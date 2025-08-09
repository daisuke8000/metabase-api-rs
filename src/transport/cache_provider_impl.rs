//! CacheProvider implementation using in-memory LRU cache
//!
//! This module provides a production implementation of the CacheProvider trait
//! using an LRU (Least Recently Used) cache for in-memory storage when the
//! "cache" feature is enabled, and a simple HashMap-based implementation otherwise.

use super::cache_traits::{CacheKey, CacheProvider, CacheStats};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
#[cfg(not(feature = "cache"))]
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Cache entry with expiration tracking
#[derive(Debug, Clone)]
struct CacheEntry {
    /// Serialized data
    data: Vec<u8>,
    /// Expiration time
    expires_at: Option<Instant>,
}

impl CacheEntry {
    /// Check if the entry is expired
    fn is_expired(&self) -> bool {
        self.expires_at
            .map(|expiry| Instant::now() > expiry)
            .unwrap_or(false)
    }
}

/// In-memory cache provider
///
/// Uses LRU cache when "cache" feature is enabled, otherwise uses a simple HashMap.
pub struct InMemoryCacheProvider {
    /// The actual cache storage
    #[cfg(feature = "cache")]
    cache: Arc<Mutex<lru::LruCache<String, CacheEntry>>>,
    #[cfg(not(feature = "cache"))]
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
    /// Maximum cache size (for non-LRU implementation)
    #[cfg(not(feature = "cache"))]
    max_size: usize,
    /// Whether caching is enabled
    enabled: bool,
    /// Cache statistics
    stats: Arc<Mutex<CacheStats>>,
}

impl InMemoryCacheProvider {
    /// Create a new in-memory cache provider with specified capacity
    pub fn new(capacity: usize) -> Self {
        #[cfg(feature = "cache")]
        {
            use std::num::NonZeroUsize;
            let capacity = NonZeroUsize::new(capacity.max(1)).unwrap();
            Self {
                cache: Arc::new(Mutex::new(lru::LruCache::new(capacity))),
                enabled: true,
                stats: Arc::new(Mutex::new(CacheStats::default())),
            }
        }
        #[cfg(not(feature = "cache"))]
        {
            Self {
                cache: Arc::new(Mutex::new(HashMap::new())),
                max_size: capacity,
                enabled: true,
                stats: Arc::new(Mutex::new(CacheStats::default())),
            }
        }
    }

    /// Create a new cache provider with default capacity (1000 items)
    pub fn with_default_capacity() -> Self {
        Self::new(1000)
    }

    /// Update hit statistics
    fn record_hit(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.hits += 1;
        }
    }

    /// Update miss statistics
    fn record_miss(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.misses += 1;
        }
    }

    /// Update cache size statistics
    fn update_size(&self, size: usize) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.size = size;
        }
    }

    /// Clean expired entries (for non-LRU implementation)
    #[cfg(not(feature = "cache"))]
    fn clean_expired(&self, cache: &mut HashMap<String, CacheEntry>) {
        cache.retain(|_, entry| !entry.is_expired());
    }

    /// Enforce size limit (for non-LRU implementation)
    #[cfg(not(feature = "cache"))]
    fn enforce_size_limit(&self, cache: &mut HashMap<String, CacheEntry>) {
        if cache.len() > self.max_size {
            // Simple eviction: remove oldest entries
            // In a real implementation, you'd track access times
            let to_remove = cache.len() - self.max_size;
            let keys: Vec<String> = cache.keys().take(to_remove).cloned().collect();
            for key in keys {
                cache.remove(&key);
            }
        }
    }
}

#[async_trait]
impl CacheProvider for InMemoryCacheProvider {
    async fn get<T>(&self, key: &impl CacheKey) -> Option<T>
    where
        T: DeserializeOwned + Send,
    {
        if !self.enabled {
            return None;
        }

        let key_str = key.to_string();

        let mut cache = self.cache.lock().ok()?;

        #[cfg(feature = "cache")]
        {
            // LRU cache implementation
            if let Some(entry) = cache.get(&key_str) {
                if entry.is_expired() {
                    cache.pop(&key_str);
                    self.record_miss();
                    self.update_size(cache.len());
                    return None;
                }

                // Deserialize the data
                if let Ok(value) = serde_json::from_slice(&entry.data) {
                    self.record_hit();
                    return Some(value);
                }
            }
        }

        #[cfg(not(feature = "cache"))]
        {
            // HashMap implementation
            if let Some(entry) = cache.get(&key_str) {
                if entry.is_expired() {
                    cache.remove(&key_str);
                    self.record_miss();
                    self.update_size(cache.len());
                    return None;
                }

                // Deserialize the data
                if let Ok(value) = serde_json::from_slice(&entry.data) {
                    self.record_hit();
                    return Some(value);
                }
            }
        }

        self.record_miss();
        None
    }

    async fn set<T>(&self, key: &impl CacheKey, value: &T, ttl: Option<Duration>) -> bool
    where
        T: Serialize + Send + Sync,
    {
        if !self.enabled {
            return false;
        }

        let key_str = key.to_string();

        // Serialize the value
        let data = match serde_json::to_vec(value) {
            Ok(data) => data,
            Err(_) => return false,
        };

        let expires_at = ttl.map(|duration| Instant::now() + duration);

        let entry = CacheEntry { data, expires_at };

        if let Ok(mut cache) = self.cache.lock() {
            #[cfg(feature = "cache")]
            {
                cache.put(key_str, entry);
                self.update_size(cache.len());
                return true;
            }

            #[cfg(not(feature = "cache"))]
            {
                self.clean_expired(&mut cache);
                cache.insert(key_str, entry);
                self.enforce_size_limit(&mut cache);
                self.update_size(cache.len());
                return true;
            }
        }

        false
    }

    async fn remove(&self, key: &impl CacheKey) -> bool {
        if !self.enabled {
            return false;
        }

        let key_str = key.to_string();

        if let Ok(mut cache) = self.cache.lock() {
            #[cfg(feature = "cache")]
            {
                let removed = cache.pop(&key_str).is_some();
                if removed {
                    self.update_size(cache.len());
                }
                return removed;
            }

            #[cfg(not(feature = "cache"))]
            {
                let removed = cache.remove(&key_str).is_some();
                if removed {
                    self.update_size(cache.len());
                }
                return removed;
            }
        }

        false
    }

    async fn clear(&self) -> bool {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
            self.update_size(0);

            // Reset stats
            if let Ok(mut stats) = self.stats.lock() {
                *stats = CacheStats::default();
            }

            return true;
        }

        false
    }

    async fn exists(&self, key: &impl CacheKey) -> bool {
        if !self.enabled {
            return false;
        }

        let key_str = key.to_string();

        if let Ok(mut cache) = self.cache.lock() {
            #[cfg(feature = "cache")]
            {
                if let Some(entry) = cache.get(&key_str) {
                    if entry.is_expired() {
                        cache.pop(&key_str);
                        self.update_size(cache.len());
                        return false;
                    }
                    return true;
                }
            }

            #[cfg(not(feature = "cache"))]
            {
                if let Some(entry) = cache.get(&key_str) {
                    if entry.is_expired() {
                        cache.remove(&key_str);
                        self.update_size(cache.len());
                        return false;
                    }
                    return true;
                }
            }
        }

        false
    }

    async fn stats(&self) -> CacheStats {
        self.stats.lock().map(|s| s.clone()).unwrap_or_default()
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            // Clear cache when disabled
            if let Ok(mut cache) = self.cache.lock() {
                cache.clear();
            }
        }
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Builder for InMemoryCacheProvider
pub struct InMemoryCacheProviderBuilder {
    capacity: usize,
    enabled: bool,
}

impl Default for InMemoryCacheProviderBuilder {
    fn default() -> Self {
        Self {
            capacity: 1000,
            enabled: true,
        }
    }
}

impl InMemoryCacheProviderBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the cache capacity
    pub fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    /// Set whether the cache is initially enabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Build the cache provider
    pub fn build(self) -> InMemoryCacheProvider {
        let mut provider = InMemoryCacheProvider::new(self.capacity);
        provider.enabled = self.enabled;
        provider
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_basic_operations() {
        let provider = InMemoryCacheProvider::new(10);

        // Test set and get
        let key = "test_key".to_string();
        let value = "test_value".to_string();

        assert!(provider.set(&key, &value, None).await);
        assert_eq!(provider.get::<String>(&key).await, Some(value.clone()));

        // Test exists
        assert!(provider.exists(&key).await);

        // Test remove
        assert!(provider.remove(&key).await);
        assert!(!provider.exists(&key).await);
        assert_eq!(provider.get::<String>(&key).await, None);
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let provider = InMemoryCacheProvider::new(10);

        let key = "expiring_key".to_string();
        let value = "expiring_value".to_string();

        // Set with very short TTL
        assert!(
            provider
                .set(&key, &value, Some(Duration::from_millis(10)))
                .await
        );

        // Should exist immediately
        assert!(provider.exists(&key).await);

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(20)).await;

        // Should be expired
        assert!(!provider.exists(&key).await);
        assert_eq!(provider.get::<String>(&key).await, None);
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let provider = InMemoryCacheProvider::new(10);

        // Add multiple items
        for i in 0..5 {
            let key = format!("key_{}", i);
            let value = format!("value_{}", i);
            assert!(provider.set(&key, &value, None).await);
        }

        // Clear cache
        assert!(provider.clear().await);

        // All items should be gone
        for i in 0..5 {
            let key = format!("key_{}", i);
            assert!(!provider.exists(&key).await);
        }
    }

    #[tokio::test]
    async fn test_cache_disabled() {
        let mut provider = InMemoryCacheProvider::new(10);

        let key = "test_key".to_string();
        let value = "test_value".to_string();

        // Disable cache
        provider.set_enabled(false);
        assert!(!provider.is_enabled());

        // Operations should fail or return None
        assert!(!provider.set(&key, &value, None).await);
        assert_eq!(provider.get::<String>(&key).await, None);
        assert!(!provider.exists(&key).await);
        assert!(!provider.remove(&key).await);
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let provider = InMemoryCacheProvider::new(10);

        let key1 = "key1".to_string();
        let key2 = "key2".to_string();
        let value = "value".to_string();

        // Set some values
        provider.set(&key1, &value, None).await;
        provider.set(&key2, &value, None).await;

        // Get existing (hit)
        provider.get::<String>(&key1).await;

        // Get non-existing (miss)
        provider.get::<String>(&"nonexistent".to_string()).await;

        let stats = provider.stats().await;
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.size, 2);
    }
}
