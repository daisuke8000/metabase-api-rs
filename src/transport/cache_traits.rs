//! Cache abstraction traits
//!
//! This module provides trait-based abstractions for caching mechanisms,
//! allowing for testable and flexible cache implementations.

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::time::Duration;

/// Cache key type - must be cloneable and debuggable
pub trait CacheKey: Clone + Debug + Send + Sync {
    /// Convert to string representation for storage
    fn to_string(&self) -> String;
}

/// Simple string-based cache key
impl CacheKey for String {
    fn to_string(&self) -> String {
        self.clone()
    }
}

/// Compound cache key for complex scenarios
#[derive(Clone, Debug)]
pub struct CompoundKey {
    /// Primary key component
    pub primary: String,
    /// Secondary key component
    pub secondary: Option<String>,
    /// Namespace for the key
    pub namespace: Option<String>,
}

impl CacheKey for CompoundKey {
    fn to_string(&self) -> String {
        let mut parts = vec![self.primary.clone()];
        if let Some(ref secondary) = self.secondary {
            parts.push(secondary.clone());
        }
        if let Some(ref namespace) = self.namespace {
            parts.insert(0, namespace.clone());
        }
        parts.join(":")
    }
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Total number of cache hits
    pub hits: u64,
    /// Total number of cache misses
    pub misses: u64,
    /// Total number of items in cache
    pub size: usize,
    /// Total bytes used (if available)
    pub bytes_used: Option<usize>,
}

impl CacheStats {
    /// Calculate hit ratio
    pub fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}

/// Cache provider trait
///
/// This trait abstracts cache operations, allowing for
/// different implementations (e.g., in-memory, Redis, mock for testing)
#[async_trait]
pub trait CacheProvider: Send + Sync {
    /// Get a value from the cache
    async fn get<T>(&self, key: &impl CacheKey) -> Option<T>
    where
        T: DeserializeOwned + Send;

    /// Set a value in the cache with optional TTL
    async fn set<T>(&self, key: &impl CacheKey, value: &T, ttl: Option<Duration>) -> bool
    where
        T: Serialize + Send + Sync;

    /// Remove a value from the cache
    async fn remove(&self, key: &impl CacheKey) -> bool;

    /// Clear all values from the cache
    async fn clear(&self) -> bool;

    /// Check if a key exists in the cache
    async fn exists(&self, key: &impl CacheKey) -> bool;

    /// Get cache statistics
    async fn stats(&self) -> CacheStats;

    /// Set cache enabled/disabled state
    fn set_enabled(&mut self, enabled: bool);

    /// Check if cache is enabled
    fn is_enabled(&self) -> bool;
}

/// Mock cache provider for testing
#[derive(Debug, Clone)]
pub struct MockCacheProvider {
    /// Whether cache operations should succeed
    pub should_succeed: bool,
    /// Whether cache is enabled
    pub enabled: bool,
    /// Mock cache hit ratio (0.0 - 1.0)
    pub hit_ratio: f64,
    /// Mock stats
    pub mock_stats: CacheStats,
}

impl Default for MockCacheProvider {
    fn default() -> Self {
        Self {
            should_succeed: true,
            enabled: true,
            hit_ratio: 0.8, // 80% hit ratio by default
            mock_stats: CacheStats {
                hits: 80,
                misses: 20,
                size: 100,
                bytes_used: Some(1024 * 10), // 10KB
            },
        }
    }
}

#[async_trait]
impl CacheProvider for MockCacheProvider {
    async fn get<T>(&self, _key: &impl CacheKey) -> Option<T>
    where
        T: DeserializeOwned + Send,
    {
        if !self.enabled || !self.should_succeed {
            return None;
        }

        // Simulate cache hit based on hit_ratio
        let random_hit = rand::random::<f64>() < self.hit_ratio;
        if random_hit {
            // Return mock data - this is a simplified mock
            // In real tests, you'd want to store and retrieve actual values
            None
        } else {
            None
        }
    }

    async fn set<T>(&self, _key: &impl CacheKey, _value: &T, _ttl: Option<Duration>) -> bool
    where
        T: Serialize + Send + Sync,
    {
        self.enabled && self.should_succeed
    }

    async fn remove(&self, _key: &impl CacheKey) -> bool {
        self.enabled && self.should_succeed
    }

    async fn clear(&self) -> bool {
        self.enabled && self.should_succeed
    }

    async fn exists(&self, _key: &impl CacheKey) -> bool {
        self.enabled && self.should_succeed
    }

    async fn stats(&self) -> CacheStats {
        self.mock_stats.clone()
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// No-op cache provider that doesn't cache anything
#[derive(Debug, Clone)]
pub struct NoOpCacheProvider;

#[async_trait]
impl CacheProvider for NoOpCacheProvider {
    async fn get<T>(&self, _key: &impl CacheKey) -> Option<T>
    where
        T: DeserializeOwned + Send,
    {
        None
    }

    async fn set<T>(&self, _key: &impl CacheKey, _value: &T, _ttl: Option<Duration>) -> bool
    where
        T: Serialize + Send + Sync,
    {
        true // Always "succeeds" but doesn't actually cache
    }

    async fn remove(&self, _key: &impl CacheKey) -> bool {
        true
    }

    async fn clear(&self) -> bool {
        true
    }

    async fn exists(&self, _key: &impl CacheKey) -> bool {
        false // Nothing is ever cached
    }

    async fn stats(&self) -> CacheStats {
        CacheStats::default()
    }

    fn set_enabled(&mut self, _enabled: bool) {
        // No-op
    }

    fn is_enabled(&self) -> bool {
        false // Always disabled
    }
}
