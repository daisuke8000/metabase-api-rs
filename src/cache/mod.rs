//! Caching layer for Metabase API client
//!
//! Provides LRU caching with TTL support for frequently accessed data.

#[cfg(feature = "cache")]
use lru::LruCache;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::num::NonZeroUsize;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::core::models::QueryResult;
use crate::Result;

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Enable or disable the cache at runtime
    pub enabled: bool,
    /// Maximum number of entries in the cache
    pub max_size: usize,
    /// Time to live for cache entries
    pub ttl: Duration,
    /// Enable metadata caching (databases, collections, cards)
    pub cache_metadata: bool,
    /// Enable query result caching
    pub cache_queries: bool,
    /// Enable session caching
    pub cache_sessions: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true, // Enabled by default for backward compatibility
            max_size: 1000,
            ttl: Duration::from_secs(300), // 5 minutes
            cache_metadata: true,
            cache_queries: false, // Disabled by default due to volatility
            cache_sessions: true,
        }
    }
}

/// Cache entry with timestamp
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    value: T,
    inserted_at: Instant,
    ttl: Duration,
}

impl<T> CacheEntry<T> {
    fn new(value: T, ttl: Duration) -> Self {
        Self {
            value,
            inserted_at: Instant::now(),
            ttl,
        }
    }

    fn is_expired(&self) -> bool {
        self.inserted_at.elapsed() > self.ttl
    }
}

/// Thread-safe cache layer
#[cfg(feature = "cache")]
#[derive(Clone)]
pub struct CacheLayer {
    metadata_cache: Arc<RwLock<LruCache<String, CacheEntry<Value>>>>,
    query_cache: Arc<RwLock<LruCache<String, CacheEntry<QueryResult>>>>,
    session_cache: Arc<RwLock<LruCache<String, CacheEntry<String>>>>,
    config: CacheConfig,
    enabled: Arc<RwLock<bool>>,
}

#[cfg(feature = "cache")]
impl std::fmt::Debug for CacheLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CacheLayer")
            .field("config", &self.config)
            .finish()
    }
}

#[cfg(feature = "cache")]
impl CacheLayer {
    /// Create a new cache layer with the given configuration
    pub fn new(config: CacheConfig) -> Self {
        let max_size =
            NonZeroUsize::new(config.max_size).unwrap_or(NonZeroUsize::new(1000).unwrap());
        let enabled = config.enabled;

        Self {
            metadata_cache: Arc::new(RwLock::new(LruCache::new(max_size))),
            query_cache: Arc::new(RwLock::new(LruCache::new(max_size))),
            session_cache: Arc::new(RwLock::new(LruCache::new(
                NonZeroUsize::new(10).unwrap(), // Limited session cache
            ))),
            config,
            enabled: Arc::new(RwLock::new(enabled)),
        }
    }

    /// Check if cache is enabled
    pub fn is_enabled(&self) -> bool {
        *self.enabled.read().unwrap()
    }

    /// Set cache enabled state
    pub fn set_enabled(&self, enabled: bool) {
        *self.enabled.write().unwrap() = enabled;
    }

    /// Get a cached metadata value
    pub fn get_metadata<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        if !self.is_enabled() || !self.config.cache_metadata {
            return None;
        }

        let cache = self.metadata_cache.read().ok()?;
        let entry = cache.peek(key)?;

        if entry.is_expired() {
            return None;
        }

        serde_json::from_value(entry.value.clone()).ok()
    }

    /// Store a metadata value in the cache
    pub fn set_metadata<T: Serialize>(&self, key: String, value: &T) -> Result<()> {
        if !self.is_enabled() || !self.config.cache_metadata {
            return Ok(());
        }

        let json_value = serde_json::to_value(value)?;
        let entry = CacheEntry::new(json_value, self.config.ttl);

        let mut cache = self.metadata_cache.write().unwrap();
        cache.put(key, entry);

        Ok(())
    }

    /// Get a cached query result
    pub fn get_query(&self, key: &str) -> Option<QueryResult> {
        if !self.is_enabled() || !self.config.cache_queries {
            return None;
        }

        let cache = self.query_cache.read().ok()?;
        let entry = cache.peek(key)?;

        if entry.is_expired() {
            return None;
        }

        Some(entry.value.clone())
    }

    /// Store a query result in the cache
    pub fn set_query(&self, key: String, value: QueryResult) -> Result<()> {
        if !self.is_enabled() || !self.config.cache_queries {
            return Ok(());
        }

        let entry = CacheEntry::new(value, self.config.ttl);

        let mut cache = self.query_cache.write().unwrap();
        cache.put(key, entry);

        Ok(())
    }

    /// Get a cached session token
    pub fn get_session(&self, key: &str) -> Option<String> {
        if !self.is_enabled() || !self.config.cache_sessions {
            return None;
        }

        let cache = self.session_cache.read().ok()?;
        let entry = cache.peek(key)?;

        if entry.is_expired() {
            return None;
        }

        Some(entry.value.clone())
    }

    /// Store a session token in the cache
    pub fn set_session(&self, key: String, token: String) -> Result<()> {
        if !self.is_enabled() || !self.config.cache_sessions {
            return Ok(());
        }

        // Sessions have longer TTL (1 hour)
        let session_ttl = Duration::from_secs(3600);
        let entry = CacheEntry::new(token, session_ttl);

        let mut cache = self.session_cache.write().unwrap();
        cache.put(key, entry);

        Ok(())
    }

    /// Invalidate a specific cache entry
    pub fn invalidate(&self, key: &str) {
        // Try to remove from all caches
        if let Ok(mut cache) = self.metadata_cache.write() {
            cache.pop(key);
        }
        if let Ok(mut cache) = self.query_cache.write() {
            cache.pop(key);
        }
        if let Ok(mut cache) = self.session_cache.write() {
            cache.pop(key);
        }
    }

    /// Clear all caches
    pub fn clear_all(&self) {
        if let Ok(mut cache) = self.metadata_cache.write() {
            cache.clear();
        }
        if let Ok(mut cache) = self.query_cache.write() {
            cache.clear();
        }
        if let Ok(mut cache) = self.session_cache.write() {
            cache.clear();
        }
    }

    /// Clear metadata cache only
    pub fn clear_metadata(&self) {
        if let Ok(mut cache) = self.metadata_cache.write() {
            cache.clear();
        }
    }

    /// Clear query cache only
    pub fn clear_queries(&self) {
        if let Ok(mut cache) = self.query_cache.write() {
            cache.clear();
        }
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let metadata_size = self.metadata_cache.read().map(|c| c.len()).unwrap_or(0);
        let query_size = self.query_cache.read().map(|c| c.len()).unwrap_or(0);
        let session_size = self.session_cache.read().map(|c| c.len()).unwrap_or(0);

        CacheStats {
            metadata_entries: metadata_size,
            query_entries: query_size,
            session_entries: session_size,
            max_size: self.config.max_size,
            ttl_seconds: self.config.ttl.as_secs(),
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub metadata_entries: usize,
    pub query_entries: usize,
    pub session_entries: usize,
    pub max_size: usize,
    pub ttl_seconds: u64,
}

/// No-op cache implementation for when cache feature is disabled
#[cfg(not(feature = "cache"))]
pub struct CacheLayer;

#[cfg(not(feature = "cache"))]
impl CacheLayer {
    pub fn new(_config: CacheConfig) -> Self {
        Self
    }

    pub fn get_metadata<T>(&self, _key: &str) -> Option<T> {
        None
    }

    pub fn set_metadata<T>(&self, _key: String, _value: &T) -> Result<()> {
        Ok(())
    }

    pub fn get_query(&self, _key: &str) -> Option<QueryResult> {
        None
    }

    pub fn set_query(&self, _key: String, _value: QueryResult) -> Result<()> {
        Ok(())
    }

    pub fn get_session(&self, _key: &str) -> Option<String> {
        None
    }

    pub fn set_session(&self, _key: String, _token: String) -> Result<()> {
        Ok(())
    }

    pub fn invalidate(&self, _key: &str) {}
    pub fn clear_all(&self) {}
    pub fn clear_metadata(&self) {}
    pub fn clear_queries(&self) {}

    pub fn stats(&self) -> CacheStats {
        CacheStats {
            metadata_entries: 0,
            query_entries: 0,
            session_entries: 0,
            max_size: 0,
            ttl_seconds: 0,
        }
    }
}

/// Generate cache key for different resource types
pub fn cache_key(resource_type: &str, id: impl std::fmt::Display) -> String {
    format!("{}:{}", resource_type, id)
}

/// Generate cache key with parameters
pub fn cache_key_with_params(
    resource_type: &str,
    id: impl std::fmt::Display,
    params: &impl Serialize,
) -> Result<String> {
    let params_hash = {
        let json = serde_json::to_string(params)?;
        // Simple hash for cache key
        let mut hash = 0u64;
        for byte in json.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    };

    Ok(format!("{}:{}:{}", resource_type, id, params_hash))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::{Card, MetabaseId};

    #[test]
    fn test_cache_key_generation() {
        let key = cache_key("card", 123);
        assert_eq!(key, "card:123");

        let key = cache_key("collection", "root");
        assert_eq!(key, "collection:root");
    }

    #[cfg(feature = "cache")]
    #[test]
    fn test_cache_operations() {
        let config = CacheConfig::default();
        let cache = CacheLayer::new(config);

        // Test metadata cache
        let card = Card {
            id: Some(crate::core::models::common::CardId(1)),
            name: "Test Card".to_string(),
            card_type: crate::core::models::CardType::Question,
            description: Some("Test description".to_string()),
            collection_id: None,
            display: "table".to_string(),
            visualization_settings: serde_json::json!({}),
            dataset_query: Some(serde_json::json!({})),
            created_at: None,
            updated_at: None,
            archived: false,
            enable_embedding: false,
            embedding_params: serde_json::json!({}),
            result_metadata: None,
            entity_id: None,
            cache_ttl: None,
            collection_position: None,
            dashboard_tab_id: None,
            dashboard_id: None,
            parameters: vec![],
            parameter_mappings: vec![],
            creator_id: None,
            database_id: None,
            table_id: None,
            query_type: None,
            public_uuid: None,
            made_public_by_id: None,
        };

        cache.set_metadata("card:1".to_string(), &card).unwrap();
        let cached: Option<Card> = cache.get_metadata("card:1");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().name, "Test Card");

        // Test cache invalidation
        cache.invalidate("card:1");
        let cached: Option<Card> = cache.get_metadata("card:1");
        assert!(cached.is_none());
    }

    #[cfg(feature = "cache")]
    #[test]
    fn test_cache_expiration() {
        use std::thread;

        let mut config = CacheConfig::default();
        config.ttl = Duration::from_millis(50); // Short TTL for testing
        let cache = CacheLayer::new(config);

        // Test metadata cache expiration
        let card = Card {
            id: Some(crate::core::models::common::CardId(1)),
            name: "Test Card".to_string(),
            card_type: crate::core::models::CardType::Question,
            description: None,
            collection_id: None,
            display: "table".to_string(),
            visualization_settings: serde_json::json!({}),
            dataset_query: Some(serde_json::json!({})),
            created_at: None,
            updated_at: None,
            archived: false,
            enable_embedding: false,
            embedding_params: serde_json::json!({}),
            result_metadata: None,
            entity_id: None,
            cache_ttl: None,
            collection_position: None,
            dashboard_tab_id: None,
            dashboard_id: None,
            parameters: vec![],
            parameter_mappings: vec![],
            creator_id: None,
            database_id: None,
            table_id: None,
            query_type: None,
            public_uuid: None,
            made_public_by_id: None,
        };

        cache.set_metadata("card:1".to_string(), &card).unwrap();

        // Should be present immediately
        let cached: Option<Card> = cache.get_metadata("card:1");
        assert!(cached.is_some());

        // Wait for expiration
        thread::sleep(Duration::from_millis(100));

        // Should be expired
        let cached: Option<Card> = cache.get_metadata("card:1");
        assert!(cached.is_none());
    }
}
