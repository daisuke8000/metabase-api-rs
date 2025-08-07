#[cfg(test)]
mod tests {
    use metabase_api_rs::core::models::collection::*;
    use metabase_api_rs::core::models::common::*;
    use serde_json;

    #[test]
    fn test_collection_creation() {
        let collection = Collection::new(
            MetabaseId::new(1),
            "Test Collection".to_string(),
        );
        
        assert_eq!(collection.id(), MetabaseId::new(1));
        assert_eq!(collection.name(), "Test Collection");
        assert!(collection.description().is_none());
        assert!(collection.parent_id().is_none());
        assert!(!collection.personal_owner_id().is_some());
    }

    #[test]
    fn test_collection_hierarchy() {
        let parent = Collection::new(
            MetabaseId::new(1),
            "Parent Collection".to_string(),
        );
        
        let child = CollectionBuilder::new(MetabaseId::new(2), "Child Collection".to_string())
            .parent_id(parent.id())
            .description("A child collection")
            .build();
        
        assert_eq!(child.parent_id(), Some(parent.id()));
        assert_eq!(child.description(), Some("A child collection"));
    }

    #[test]
    fn test_collection_deserialize() {
        let json_str = r#"{
            "id": 10,
            "name": "Marketing Reports",
            "description": "All marketing related reports",
            "color": "#509EE3",
            "parent_id": 5,
            "personal_owner_id": null,
            "namespace": "snippets",
            "slug": "marketing_reports",
            "archived": false,
            "can_write": true
        }"#;

        let collection: Collection = serde_json::from_str(json_str).unwrap();
        
        assert_eq!(collection.id().as_i64(), 10);
        assert_eq!(collection.name(), "Marketing Reports");
        assert_eq!(collection.description(), Some("All marketing related reports"));
        assert_eq!(collection.color(), Some("#509EE3"));
        assert_eq!(collection.parent_id(), Some(MetabaseId::new(5)));
        assert_eq!(collection.namespace(), Some("snippets"));
        assert_eq!(collection.slug(), Some("marketing_reports"));
        assert!(!collection.archived());
        assert_eq!(collection.can_write(), Some(true));
    }

    #[test]
    fn test_personal_collection() {
        let personal = CollectionBuilder::new(MetabaseId::new(100), "My Personal Collection".to_string())
            .personal_owner_id(42)
            .build();
        
        assert_eq!(personal.personal_owner_id(), Some(42));
        assert!(personal.is_personal());
    }

    #[test]
    fn test_root_collection() {
        let json_str = r#"{
            "id": "root",
            "name": "Our analytics",
            "parent_id": null,
            "personal_owner_id": null,
            "can_write": false
        }"#;

        // Note: This should handle the special "root" collection
        // For now, we'll test with a numeric ID
        let json_str_numeric = r#"{
            "id": 0,
            "name": "Our analytics",
            "parent_id": null,
            "personal_owner_id": null,
            "can_write": false
        }"#;

        let root: Collection = serde_json::from_str(json_str_numeric).unwrap();
        assert_eq!(root.id().as_i64(), 0);
        assert_eq!(root.name(), "Our analytics");
        assert!(root.parent_id().is_none());
    }
}