//! Unit tests for cache functionality

#[cfg(feature = "cache")]
mod cache_tests {
    use metabase_api_rs::cache::{CacheConfig, CacheLayer, cache_key, cache_key_with_params};
    use metabase_api_rs::models::{Card, Collection, Dashboard, MetabaseId};
    use serde_json::json;
    use std::time::Duration;

    #[test]
    fn test_cache_key_generation() {
        // Simple cache key
        let key = cache_key("card", 123);
        assert_eq!(key, "card:123");

        // Collection cache key
        let key = cache_key("collection", "root");
        assert_eq!(key, "collection:root");

        // Dashboard cache key
        let key = cache_key("dashboard", 456);
        assert_eq!(key, "dashboard:456");
    }

    #[test]
    fn test_cache_key_with_params() {
        let params = json!({
            "filter": "active",
            "sort": "name"
        });
        
        let key = cache_key_with_params("card", 123, &params).unwrap();
        assert!(key.starts_with("card:123:"));
        
        // Same parameters should generate same hash
        let key2 = cache_key_with_params("card", 123, &params).unwrap();
        assert_eq!(key, key2);
        
        // Different parameters should generate different hash
        let params2 = json!({
            "filter": "archived",
            "sort": "date"
        });
        let key3 = cache_key_with_params("card", 123, &params2).unwrap();
        assert_ne!(key, key3);
    }

    #[test]
    fn test_cache_layer_creation() {
        let config = CacheConfig {
            max_size: 500,
            ttl: Duration::from_secs(600),
            cache_metadata: true,
            cache_queries: false,
            cache_sessions: true,
        };
        
        let cache = CacheLayer::new(config);
        let stats = cache.stats();
        
        assert_eq!(stats.max_size, 500);
        assert_eq!(stats.ttl_seconds, 600);
        assert_eq!(stats.metadata_entries, 0);
        assert_eq!(stats.query_entries, 0);
        assert_eq!(stats.session_entries, 0);
    }

    #[test]
    fn test_metadata_cache() {
        let config = CacheConfig::default();
        let cache = CacheLayer::new(config);
        
        // Create test card
        let card = Card {
            id: MetabaseId(1),
            name: "Test Card".to_string(),
            card_type: metabase_api_rs::models::CardType::Question,
            description: Some("Test description".to_string()),
            collection_id: None,
            display: "table".to_string(),
            visualization_settings: json!({}),
            dataset_query: Some(json!({})),
            created_at: None,
            updated_at: None,
            archived: false,
            enable_embedding: false,
            embedding_params: json!({}),
            result_metadata: None,
            entity_id: None,
            cache_ttl: None,
            collection_position: None,
            dashboard_tab_id: None,
            dashboard_id: None,
            parameters: vec![],
            parameter_mappings: vec![],
        };
        
        // Store in cache
        let key = "card:1".to_string();
        cache.set_metadata(key.clone(), &card).unwrap();
        
        // Retrieve from cache
        let cached: Option<Card> = cache.get_metadata(&key);
        assert!(cached.is_some());
        
        let cached_card = cached.unwrap();
        assert_eq!(cached_card.id.0, 1);
        assert_eq!(cached_card.name, "Test Card");
        
        // Check cache stats
        let stats = cache.stats();
        assert_eq!(stats.metadata_entries, 1);
    }

    #[test]
    fn test_collection_cache() {
        let config = CacheConfig::default();
        let cache = CacheLayer::new(config);
        
        // Create test collection
        let collection = Collection {
            id: MetabaseId(2),
            name: "Test Collection".to_string(),
            slug: Some("test-collection".to_string()),
            color: Some("#000000".to_string()),
            description: Some("Test collection description".to_string()),
            personal_owner_id: None,
            archived: false,
            location: Some("/".to_string()),
            namespace: None,
            created_at: None,
            authority_level: None,
        };
        
        // Store in cache
        let key = "collection:2".to_string();
        cache.set_metadata(key.clone(), &collection).unwrap();
        
        // Retrieve from cache
        let cached: Option<Collection> = cache.get_metadata(&key);
        assert!(cached.is_some());
        
        let cached_collection = cached.unwrap();
        assert_eq!(cached_collection.id.0, 2);
        assert_eq!(cached_collection.name, "Test Collection");
    }

    #[test]
    fn test_cache_invalidation() {
        let config = CacheConfig::default();
        let cache = CacheLayer::new(config);
        
        // Create and cache a dashboard
        let dashboard = Dashboard {
            id: MetabaseId(3),
            name: "Test Dashboard".to_string(),
            description: Some("Test dashboard description".to_string()),
            archived: false,
            collection_id: None,
            collection_position: None,
            created_at: None,
            updated_at: None,
            parameters: vec![],
            dashcards: vec![],
            enable_embedding: false,
            embedding_params: None,
            can_write: false,
            public_uuid: None,
        };
        
        let key = "dashboard:3".to_string();
        cache.set_metadata(key.clone(), &dashboard).unwrap();
        
        // Verify it's cached
        let cached: Option<Dashboard> = cache.get_metadata(&key);
        assert!(cached.is_some());
        
        // Invalidate the cache entry
        cache.invalidate(&key);
        
        // Verify it's no longer in cache
        let cached: Option<Dashboard> = cache.get_metadata(&key);
        assert!(cached.is_none());
        
        // Check stats
        let stats = cache.stats();
        assert_eq!(stats.metadata_entries, 0);
    }

    #[test]
    fn test_cache_clear() {
        let config = CacheConfig::default();
        let cache = CacheLayer::new(config);
        
        // Add multiple items to cache
        let card = Card {
            id: MetabaseId(1),
            name: "Card 1".to_string(),
            card_type: metabase_api_rs::models::CardType::Question,
            description: None,
            collection_id: None,
            display: "table".to_string(),
            visualization_settings: json!({}),
            dataset_query: Some(json!({})),
            created_at: None,
            updated_at: None,
            archived: false,
            enable_embedding: false,
            embedding_params: json!({}),
            result_metadata: None,
            entity_id: None,
            cache_ttl: None,
            collection_position: None,
            dashboard_tab_id: None,
            dashboard_id: None,
            parameters: vec![],
            parameter_mappings: vec![],
        };
        
        let collection = Collection {
            id: MetabaseId(2),
            name: "Collection 1".to_string(),
            slug: None,
            color: None,
            description: None,
            personal_owner_id: None,
            archived: false,
            location: None,
            namespace: None,
            created_at: None,
            authority_level: None,
        };
        
        cache.set_metadata("card:1".to_string(), &card).unwrap();
        cache.set_metadata("collection:2".to_string(), &collection).unwrap();
        
        // Verify items are cached
        let stats = cache.stats();
        assert_eq!(stats.metadata_entries, 2);
        
        // Clear metadata cache
        cache.clear_metadata();
        
        // Verify cache is empty
        let stats = cache.stats();
        assert_eq!(stats.metadata_entries, 0);
        
        let cached_card: Option<Card> = cache.get_metadata("card:1");
        assert!(cached_card.is_none());
        
        let cached_collection: Option<Collection> = cache.get_metadata("collection:2");
        assert!(cached_collection.is_none());
    }

    #[test]
    fn test_session_cache() {
        let config = CacheConfig::default();
        let cache = CacheLayer::new(config);
        
        // Store session token
        let token = "test-session-token".to_string();
        cache.set_session("user:123".to_string(), token.clone()).unwrap();
        
        // Retrieve session token
        let cached_token = cache.get_session("user:123");
        assert!(cached_token.is_some());
        assert_eq!(cached_token.unwrap(), token);
        
        // Check stats
        let stats = cache.stats();
        assert_eq!(stats.session_entries, 1);
    }

    #[test]
    fn test_cache_expiration() {
        use std::thread;
        
        // Create cache with very short TTL
        let config = CacheConfig {
            max_size: 100,
            ttl: Duration::from_millis(50), // 50ms TTL for testing
            cache_metadata: true,
            cache_queries: false,
            cache_sessions: true,
        };
        
        let cache = CacheLayer::new(config);
        
        // Store a card
        let card = Card {
            id: MetabaseId(1),
            name: "Expiring Card".to_string(),
            card_type: metabase_api_rs::models::CardType::Question,
            description: None,
            collection_id: None,
            display: "table".to_string(),
            visualization_settings: json!({}),
            dataset_query: Some(json!({})),
            created_at: None,
            updated_at: None,
            archived: false,
            enable_embedding: false,
            embedding_params: json!({}),
            result_metadata: None,
            entity_id: None,
            cache_ttl: None,
            collection_position: None,
            dashboard_tab_id: None,
            dashboard_id: None,
            parameters: vec![],
            parameter_mappings: vec![],
        };
        
        cache.set_metadata("card:1".to_string(), &card).unwrap();
        
        // Should be present immediately
        let cached: Option<Card> = cache.get_metadata("card:1");
        assert!(cached.is_some());
        
        // Wait for expiration
        thread::sleep(Duration::from_millis(100));
        
        // Should be expired now
        let cached: Option<Card> = cache.get_metadata("card:1");
        assert!(cached.is_none());
    }

    #[test]
    fn test_cache_disabled_flags() {
        // Test with metadata caching disabled
        let config = CacheConfig {
            cache_metadata: false, // Disabled
            cache_queries: true,
            cache_sessions: true,
            ..Default::default()
        };
        
        let cache = CacheLayer::new(config);
        
        let card = Card {
            id: MetabaseId(1),
            name: "Test Card".to_string(),
            card_type: metabase_api_rs::models::CardType::Question,
            description: None,
            collection_id: None,
            display: "table".to_string(),
            visualization_settings: json!({}),
            dataset_query: Some(json!({})),
            created_at: None,
            updated_at: None,
            archived: false,
            enable_embedding: false,
            embedding_params: json!({}),
            result_metadata: None,
            entity_id: None,
            cache_ttl: None,
            collection_position: None,
            dashboard_tab_id: None,
            dashboard_id: None,
            parameters: vec![],
            parameter_mappings: vec![],
        };
        
        // Try to cache - should succeed but not actually store
        cache.set_metadata("card:1".to_string(), &card).unwrap();
        
        // Try to retrieve - should return None since caching is disabled
        let cached: Option<Card> = cache.get_metadata("card:1");
        assert!(cached.is_none());
    }
}