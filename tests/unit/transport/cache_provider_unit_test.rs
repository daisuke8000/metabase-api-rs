//! Unit tests for CacheProvider implementations

use metabase_api_rs::transport::{
    CacheKey, CacheProvider, CacheStats, CompoundKey, InMemoryCacheProvider,
    InMemoryCacheProviderBuilder, MockCacheProvider, NoOpCacheProvider,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Test data structure for cache testing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestData {
    id: u32,
    name: String,
    value: f64,
}

impl TestData {
    fn new(id: u32, name: impl Into<String>, value: f64) -> Self {
        Self {
            id,
            name: name.into(),
            value,
        }
    }
}

/// Test complex nested structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct ComplexData {
    data: TestData,
    metadata: std::collections::HashMap<String, String>,
    timestamp: i64,
}

mod cache_key_tests {
    use super::*;

    #[test]
    fn test_string_cache_key() {
        let key = "test_key".to_string();
        assert_eq!(key.to_string(), "test_key");
    }

    #[test]
    fn test_compound_cache_key() {
        let key = CompoundKey {
            primary: "user".to_string(),
            secondary: Some("123".to_string()),
            namespace: None,
        };
        assert_eq!(key.to_string(), "user:123");

        let key_with_namespace = CompoundKey {
            primary: "user".to_string(),
            secondary: Some("123".to_string()),
            namespace: Some("auth".to_string()),
        };
        assert_eq!(key_with_namespace.to_string(), "auth:user:123");

        let key_primary_only = CompoundKey {
            primary: "global_setting".to_string(),
            secondary: None,
            namespace: None,
        };
        assert_eq!(key_primary_only.to_string(), "global_setting");
    }
}

mod cache_stats_tests {
    use super::*;

    #[test]
    fn test_cache_stats_hit_ratio() {
        let mut stats = CacheStats::default();
        assert_eq!(stats.hit_ratio(), 0.0);

        stats.hits = 80;
        stats.misses = 20;
        assert_eq!(stats.hit_ratio(), 0.8);

        stats.hits = 0;
        stats.misses = 100;
        assert_eq!(stats.hit_ratio(), 0.0);

        stats.hits = 100;
        stats.misses = 0;
        assert_eq!(stats.hit_ratio(), 1.0);
    }
}

mod in_memory_cache_provider_tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_set_and_get() {
        let provider = InMemoryCacheProvider::new(100);

        let key = "test_key".to_string();
        let data = TestData::new(1, "test", 42.0);

        // Set value
        assert!(provider.set(&key, &data, None).await);

        // Get value
        let retrieved: Option<TestData> = provider.get(&key).await;
        assert_eq!(retrieved, Some(data));
    }

    #[tokio::test]
    async fn test_get_nonexistent_key() {
        let provider = InMemoryCacheProvider::new(100);

        let key = "nonexistent".to_string();
        let retrieved: Option<TestData> = provider.get(&key).await;
        assert_eq!(retrieved, None);
    }

    #[tokio::test]
    async fn test_remove() {
        let provider = InMemoryCacheProvider::new(100);

        let key = "remove_test".to_string();
        let data = TestData::new(2, "remove", 99.9);

        // Set value
        assert!(provider.set(&key, &data, None).await);
        assert!(provider.exists(&key).await);

        // Remove value
        assert!(provider.remove(&key).await);
        assert!(!provider.exists(&key).await);

        // Try to get removed value
        let retrieved: Option<TestData> = provider.get(&key).await;
        assert_eq!(retrieved, None);
    }

    #[tokio::test]
    async fn test_clear() {
        let provider = InMemoryCacheProvider::new(100);

        // Add multiple items
        for i in 0..10 {
            let key = format!("key_{}", i);
            let data = TestData::new(i, format!("item_{}", i), i as f64);
            assert!(provider.set(&key, &data, None).await);
        }

        // Verify all exist
        for i in 0..10 {
            let key = format!("key_{}", i);
            assert!(provider.exists(&key).await);
        }

        // Clear all
        assert!(provider.clear().await);

        // Verify all are gone
        for i in 0..10 {
            let key = format!("key_{}", i);
            assert!(!provider.exists(&key).await);
        }
    }

    #[tokio::test]
    async fn test_ttl_expiration() {
        let provider = InMemoryCacheProvider::new(100);

        let key = "ttl_test".to_string();
        let data = TestData::new(3, "ttl", 77.7);

        // Set with very short TTL
        assert!(
            provider
                .set(&key, &data, Some(Duration::from_millis(50)))
                .await
        );

        // Should exist immediately
        assert!(provider.exists(&key).await);
        let retrieved: Option<TestData> = provider.get(&key).await;
        assert_eq!(retrieved, Some(data.clone()));

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Should be expired
        assert!(!provider.exists(&key).await);
        let retrieved: Option<TestData> = provider.get(&key).await;
        assert_eq!(retrieved, None);
    }

    #[tokio::test]
    async fn test_lru_eviction() {
        // Small cache with only 3 items capacity
        let provider = InMemoryCacheProvider::new(3);

        // Add 3 items
        for i in 0..3 {
            let key = format!("lru_{}", i);
            let data = TestData::new(i, format!("lru_{}", i), i as f64);
            assert!(provider.set(&key, &data, None).await);
        }

        // All 3 should exist
        for i in 0..3 {
            let key = format!("lru_{}", i);
            assert!(provider.exists(&key).await);
        }

        // Add a 4th item, should evict the least recently used (lru_0)
        let key = "lru_3".to_string();
        let data = TestData::new(3, "lru_3", 3.0);
        assert!(provider.set(&key, &data, None).await);

        // lru_0 should be evicted
        assert!(!provider.exists(&"lru_0".to_string()).await);

        // Others should still exist
        assert!(provider.exists(&"lru_1".to_string()).await);
        assert!(provider.exists(&"lru_2".to_string()).await);
        assert!(provider.exists(&"lru_3".to_string()).await);
    }

    #[tokio::test]
    async fn test_cache_disabled() {
        let mut provider = InMemoryCacheProvider::new(100);

        let key = "disabled_test".to_string();
        let data = TestData::new(4, "disabled", 11.11);

        // Initially enabled
        assert!(provider.is_enabled());
        assert!(provider.set(&key, &data, None).await);
        assert!(provider.exists(&key).await);

        // Disable cache
        provider.set_enabled(false);
        assert!(!provider.is_enabled());

        // Operations should fail
        let key2 = "disabled_test2".to_string();
        assert!(!provider.set(&key2, &data, None).await);
        assert!(!provider.exists(&key2).await);
        assert!(!provider.remove(&key2).await);

        let retrieved: Option<TestData> = provider.get(&key2).await;
        assert_eq!(retrieved, None);

        // Original cached item should also be inaccessible
        assert!(!provider.exists(&key).await);
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let provider = InMemoryCacheProvider::new(100);

        // Initial stats
        let stats = provider.stats().await;
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);

        let key1 = "stats_test1".to_string();
        let key2 = "stats_test2".to_string();
        let data = TestData::new(5, "stats", 55.55);

        // Set some values
        provider.set(&key1, &data, None).await;
        provider.set(&key2, &data, None).await;

        // Get existing key (hit)
        let _: Option<TestData> = provider.get(&key1).await;

        // Get non-existing key (miss)
        let _: Option<TestData> = provider.get(&"nonexistent".to_string()).await;

        // Check stats
        let stats = provider.stats().await;
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.size, 2);
        assert_eq!(stats.hit_ratio(), 0.5);
    }

    #[tokio::test]
    async fn test_complex_data_types() {
        let provider = InMemoryCacheProvider::new(100);

        let key = "complex".to_string();
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("type".to_string(), "complex".to_string());
        metadata.insert("version".to_string(), "1.0".to_string());

        let data = ComplexData {
            data: TestData::new(100, "complex_test", 999.99),
            metadata,
            timestamp: 1234567890,
        };

        // Set and get complex data
        assert!(provider.set(&key, &data, None).await);
        let retrieved: Option<ComplexData> = provider.get(&key).await;
        assert_eq!(retrieved, Some(data));
    }

    #[tokio::test]
    async fn test_builder_pattern() {
        let provider = InMemoryCacheProviderBuilder::new()
            .capacity(50)
            .enabled(false)
            .build();

        assert!(!provider.is_enabled());

        let key = "builder_test".to_string();
        let data = TestData::new(6, "builder", 66.66);

        // Should fail because cache is disabled
        assert!(!provider.set(&key, &data, None).await);
    }

    #[tokio::test]
    async fn test_default_capacity() {
        let provider = InMemoryCacheProvider::with_default_capacity();

        // Should be able to store many items (default is 1000)
        for i in 0..100 {
            let key = format!("default_{}", i);
            let data = TestData::new(i, format!("default_{}", i), i as f64);
            assert!(provider.set(&key, &data, None).await);
        }

        // All should still exist (well under capacity)
        for i in 0..100 {
            let key = format!("default_{}", i);
            assert!(provider.exists(&key).await);
        }
    }
}

mod mock_cache_provider_tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_provider_success() {
        let provider = MockCacheProvider::default();

        let key = "mock_test".to_string();
        let data = TestData::new(1, "mock", 11.11);

        // Operations should succeed by default
        assert!(provider.set(&key, &data, None).await);
        assert!(provider.exists(&key).await);
        assert!(provider.remove(&key).await);
        assert!(provider.clear().await);
    }

    #[tokio::test]
    async fn test_mock_provider_failure() {
        let provider = MockCacheProvider {
            should_succeed: false,
            enabled: true,
            hit_ratio: 0.0,
            mock_stats: CacheStats::default(),
        };

        let key = "mock_fail".to_string();
        let data = TestData::new(2, "fail", 22.22);

        // Operations should fail when should_succeed is false
        assert!(!provider.set(&key, &data, None).await);
        assert!(!provider.exists(&key).await);
        assert!(!provider.remove(&key).await);
        assert!(!provider.clear().await);
    }

    #[tokio::test]
    async fn test_mock_provider_disabled() {
        let mut provider = MockCacheProvider::default();
        provider.set_enabled(false);

        let key = "mock_disabled".to_string();
        let data = TestData::new(3, "disabled", 33.33);

        // Operations should fail when disabled
        assert!(!provider.set(&key, &data, None).await);
        assert!(!provider.exists(&key).await);
        assert!(!provider.remove(&key).await);
        assert!(!provider.clear().await);
    }

    #[tokio::test]
    async fn test_mock_provider_stats() {
        let provider = MockCacheProvider {
            should_succeed: true,
            enabled: true,
            hit_ratio: 0.75,
            mock_stats: CacheStats {
                hits: 75,
                misses: 25,
                size: 50,
                bytes_used: Some(2048),
            },
        };

        let stats = provider.stats().await;
        assert_eq!(stats.hits, 75);
        assert_eq!(stats.misses, 25);
        assert_eq!(stats.size, 50);
        assert_eq!(stats.bytes_used, Some(2048));
        assert_eq!(stats.hit_ratio(), 0.75);
    }
}

mod noop_cache_provider_tests {
    use super::*;

    #[tokio::test]
    async fn test_noop_provider() {
        let provider = NoOpCacheProvider;

        let key = "noop_test".to_string();
        let data = TestData::new(1, "noop", 44.44);

        // Set should "succeed" but not actually cache
        assert!(provider.set(&key, &data, None).await);

        // Get should always return None
        let retrieved: Option<TestData> = provider.get(&key).await;
        assert_eq!(retrieved, None);

        // Exists should always return false
        assert!(!provider.exists(&key).await);

        // Remove and clear should "succeed"
        assert!(provider.remove(&key).await);
        assert!(provider.clear().await);

        // Stats should be empty
        let stats = provider.stats().await;
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.size, 0);

        // Should always be disabled
        assert!(!provider.is_enabled());
    }

    #[tokio::test]
    async fn test_noop_provider_immutable() {
        let mut provider = NoOpCacheProvider;

        // Setting enabled should have no effect
        provider.set_enabled(true);
        assert!(!provider.is_enabled());

        provider.set_enabled(false);
        assert!(!provider.is_enabled());
    }
}