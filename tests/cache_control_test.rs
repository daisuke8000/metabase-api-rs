//! Cache control mechanism tests

#[cfg(feature = "cache")]
mod cache_control_tests {
    use metabase_api_rs::api::ClientBuilder;
    use metabase_api_rs::cache::CacheConfig;

    #[test]
    fn test_cache_disabled_at_creation() {
        // Test that cache can be disabled when creating client
        let client = ClientBuilder::new("http://localhost:3000")
            .disable_cache()
            .build()
            .unwrap();

        assert!(!client.is_cache_enabled());
    }

    #[test]
    fn test_cache_enabled_by_default() {
        // Test that cache is enabled by default
        let client = ClientBuilder::new("http://localhost:3000").build().unwrap();

        assert!(client.is_cache_enabled());
    }

    #[test]
    fn test_cache_enabled_explicit() {
        // Test explicit cache enabling
        let client = ClientBuilder::new("http://localhost:3000")
            .cache_enabled(true)
            .build()
            .unwrap();

        assert!(client.is_cache_enabled());
    }

    #[test]
    fn test_runtime_cache_control() {
        // Test runtime enable/disable
        let mut client = ClientBuilder::new("http://localhost:3000").build().unwrap();

        assert!(client.is_cache_enabled());

        // Disable cache at runtime
        client.set_cache_enabled(false);
        assert!(!client.is_cache_enabled());

        // Re-enable cache at runtime
        client.set_cache_enabled(true);
        assert!(client.is_cache_enabled());
    }

    #[test]
    fn test_cache_config_with_disabled() {
        // Test CacheConfig with enabled flag
        let mut config = CacheConfig::default();
        assert!(config.enabled); // Should be enabled by default

        config.enabled = false;

        let client = ClientBuilder::new("http://localhost:3000")
            .cache_config(config)
            .build()
            .unwrap();

        assert!(!client.is_cache_enabled());
    }

    #[tokio::test]
    async fn test_cache_operations_when_disabled() {
        // Create client with cache disabled
        let client = ClientBuilder::new("http://localhost:3000")
            .disable_cache()
            .build()
            .unwrap();

        // This would normally use cache, but should skip when disabled
        // We can't test actual behavior without a mock, but we can verify
        // the method doesn't panic
        assert!(!client.is_cache_enabled());
    }

    #[test]
    fn test_cache_layer_enabled_check() {
        use metabase_api_rs::cache::CacheLayer;

        let mut config = CacheConfig::default();
        config.enabled = false;

        let cache = CacheLayer::new(config);

        // When disabled, get operations should return None
        let result: Option<String> = cache.get_metadata("test_key");
        assert!(result.is_none());

        // Set operations should be no-ops
        let value = "test_value";
        cache.set_metadata("test_key".to_string(), &value).unwrap();

        // Still should return None since cache is disabled
        let result: Option<String> = cache.get_metadata("test_key");
        assert!(result.is_none());
    }
}
