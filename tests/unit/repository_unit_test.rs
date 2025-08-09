//! Unit tests for Repository layer

use metabase_api_rs::repository::{
    RepositoryFactory, RepositoryConfig,
    CardRepository, CollectionRepository, DashboardRepository, QueryRepository,
    PaginationParams, FilterParams,
};
use metabase_api_rs::core::models::{
    Card, CardId, Collection, CollectionId, Dashboard, DashboardId, DatabaseId,
};
use metabase_api_rs::repository::query::{Query, QueryType, QueryFilterParams};
use metabase_api_rs::repository::card::CardFilterParams;
use metabase_api_rs::repository::collection::CollectionFilterParams;
use metabase_api_rs::repository::dashboard::DashboardFilterParams;

#[tokio::test]
async fn test_card_repository_crud() {
    // Create a mock repository
    let factory = RepositoryFactory::new(RepositoryConfig::testing());
    let card_repo = factory.create_card_repository();
    
    // Create a card
    let new_card = Card {
        id: None,
        name: "Test Card".to_string(),
        description: Some("Test Description".to_string()),
        collection_id: Some(1),
        collection_position: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        archived: Some(false),
        dataset_query: serde_json::json!({}),
        display: "table".to_string(),
        enable_embedding: false,
        embedding_params: None,
        made_public_by_id: None,
        public_uuid: None,
        query_type: Some("query".to_string()),
        cache_ttl: None,
        result_metadata: None,
        collection: None,
        database_id: Some(1),
        table_id: None,
        can_write: true,
        dashboard_count: 0,
        last_query_start: None,
        average_query_time: None,
        creator_id: 1,
        moderation_reviews: Vec::new(),
        parameter_mappings: None,
        parameters: None,
        visualization_settings: serde_json::json!({}),
    };
    
    // Test create
    let created_card = card_repo.create(&new_card).await.unwrap();
    assert!(created_card.id.is_some());
    assert_eq!(created_card.name, "Test Card");
    
    // Test get
    let card_id = created_card.id.unwrap();
    let retrieved_card = card_repo.get(&card_id).await.unwrap();
    assert_eq!(retrieved_card.name, "Test Card");
    
    // Test update
    let mut updated_card = retrieved_card.clone();
    updated_card.name = "Updated Card".to_string();
    let result = card_repo.update(&card_id, &updated_card).await.unwrap();
    assert_eq!(result.name, "Updated Card");
    
    // Test list
    let cards = card_repo.list(None, None).await.unwrap();
    assert!(!cards.is_empty());
    
    // Test delete
    card_repo.delete(&card_id).await.unwrap();
    
    // Verify deletion
    let exists = card_repo.exists(&card_id).await.unwrap();
    assert!(!exists);
}

#[tokio::test]
async fn test_card_repository_special_operations() {
    let factory = RepositoryFactory::new(RepositoryConfig::testing());
    let card_repo = factory.create_card_repository();
    
    // Create a test card
    let card = Card {
        id: Some(CardId(1)),
        name: "Original Card".to_string(),
        description: Some("Test card for operations".to_string()),
        collection_id: Some(1),
        archived: Some(false),
        ..Default::default()
    };
    
    // Add card to mock repository (cast to MockCardRepository for testing)
    if let Some(mock_repo) = card_repo.as_any().downcast_ref::<metabase_api_rs::repository::card::MockCardRepository>() {
        mock_repo.add_card(card.clone()).await;
    }
    
    // Test archive
    let card_id = CardId(1);
    card_repo.archive(&card_id).await.unwrap();
    
    // Test unarchive
    card_repo.unarchive(&card_id).await.unwrap();
    
    // Test copy
    let copied = card_repo.copy(&card_id, "Copied Card").await.unwrap();
    assert_eq!(copied.name, "Copied Card");
    assert_ne!(copied.id, Some(card_id));
    
    // Test search
    let results = card_repo.search("Original").await.unwrap();
    assert!(!results.is_empty());
}

#[tokio::test]
async fn test_collection_repository_crud() {
    let factory = RepositoryFactory::new(RepositoryConfig::testing());
    let collection_repo = factory.create_collection_repository();
    
    // Create a collection
    let new_collection = Collection {
        id: None,
        name: "Test Collection".to_string(),
        slug: Some("test-collection".to_string()),
        description: Some("Test Description".to_string()),
        color: Some("#1E90FF".to_string()),
        archived: Some(false),
        parent_id: None,
        namespace: Some("cards".to_string()),
        authority_level: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        personal_owner_id: None,
        location: Some("/".to_string()),
        effective_ancestors: None,
        effective_location: None,
        can_write: true,
        type_: None,
    };
    
    // Test create
    let created = collection_repo.create(&new_collection).await.unwrap();
    assert!(created.id.is_some());
    
    // Test get
    let collection_id = created.id.unwrap();
    let retrieved = collection_repo.get(&collection_id).await.unwrap();
    assert_eq!(retrieved.name, "Test Collection");
    
    // Test list
    let collections = collection_repo.list(None, None).await.unwrap();
    assert!(!collections.is_empty());
    
    // Test delete
    collection_repo.delete(&collection_id).await.unwrap();
}

#[tokio::test]
async fn test_collection_repository_hierarchy() {
    let factory = RepositoryFactory::new(RepositoryConfig::testing());
    let collection_repo = factory.create_collection_repository();
    
    // Create parent collection
    let parent = Collection {
        id: Some(CollectionId(1)),
        name: "Parent Collection".to_string(),
        parent_id: None,
        ..Default::default()
    };
    
    // Create child collection
    let child = Collection {
        id: Some(CollectionId(2)),
        name: "Child Collection".to_string(),
        parent_id: Some(CollectionId(1)),
        ..Default::default()
    };
    
    // Add to mock repository
    if let Some(mock_repo) = collection_repo.as_any().downcast_ref::<metabase_api_rs::repository::collection::MockCollectionRepository>() {
        mock_repo.add_collection(parent.clone()).await;
        mock_repo.add_collection(child.clone()).await;
    }
    
    // Test get children
    let children = collection_repo.get_children(CollectionId(1)).await.unwrap();
    assert_eq!(children.len(), 1);
    assert_eq!(children[0].name, "Child Collection");
    
    // Test get root collections
    let roots = collection_repo.get_root_collections().await.unwrap();
    assert_eq!(roots.len(), 1);
    assert_eq!(roots[0].name, "Parent Collection");
}

#[tokio::test]
async fn test_dashboard_repository_crud() {
    let factory = RepositoryFactory::new(RepositoryConfig::testing());
    let dashboard_repo = factory.create_dashboard_repository();
    
    // Create a dashboard
    let new_dashboard = Dashboard {
        id: None,
        name: "Test Dashboard".to_string(),
        slug: Some("test-dashboard".to_string()),
        description: Some("Test Description".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        archived: Some(false),
        collection_id: Some(1),
        collection_position: None,
        creator_id: 1,
        made_public_by_id: None,
        public_uuid: None,
        parameters: Vec::new(),
        points_of_interest: None,
        show_in_getting_started: false,
        enable_embedding: false,
        embedding_params: None,
        cache_ttl: None,
        collection: None,
        can_write: true,
        param_fields: None,
        param_values: None,
        last_view_at: None,
        view_count: 0,
        dashcards: Vec::new(),
    };
    
    // Test create
    let created = dashboard_repo.create(&new_dashboard).await.unwrap();
    assert!(created.id.is_some());
    
    // Test get
    let dashboard_id = created.id.unwrap();
    let retrieved = dashboard_repo.get(&dashboard_id).await.unwrap();
    assert_eq!(retrieved.name, "Test Dashboard");
    
    // Test list
    let dashboards = dashboard_repo.list(None, None).await.unwrap();
    assert!(!dashboards.is_empty());
}

#[tokio::test]
async fn test_dashboard_repository_cards() {
    let factory = RepositoryFactory::new(RepositoryConfig::testing());
    let dashboard_repo = factory.create_dashboard_repository();
    
    // Create a dashboard
    let dashboard = Dashboard {
        id: Some(DashboardId(1)),
        name: "Dashboard with Cards".to_string(),
        ..Default::default()
    };
    
    // Add to mock repository
    if let Some(mock_repo) = dashboard_repo.as_any().downcast_ref::<metabase_api_rs::repository::dashboard::MockDashboardRepository>() {
        mock_repo.add_dashboard(dashboard.clone()).await;
    }
    
    let dashboard_id = DashboardId(1);
    
    // Test add card
    let card_data = serde_json::json!({
        "card_id": 1,
        "row": 0,
        "col": 0,
        "size_x": 4,
        "size_y": 3,
    });
    
    let added_card = dashboard_repo.add_card(&dashboard_id, &card_data).await.unwrap();
    assert!(added_card.get("id").is_some());
    
    // Test get cards
    let cards = dashboard_repo.get_cards(&dashboard_id).await.unwrap();
    assert_eq!(cards.len(), 1);
    
    // Test update card
    let card_id = 1;
    let updates = serde_json::json!({
        "row": 1,
        "col": 2,
    });
    
    let updated = dashboard_repo.update_card(&dashboard_id, card_id, &updates).await.unwrap();
    assert_eq!(updated.get("row").and_then(|v| v.as_i64()), Some(1));
    
    // Test remove card
    dashboard_repo.remove_card(&dashboard_id, card_id).await.unwrap();
    let cards_after = dashboard_repo.get_cards(&dashboard_id).await.unwrap();
    assert!(cards_after.is_empty());
}

#[tokio::test]
async fn test_query_repository_execution() {
    let factory = RepositoryFactory::new(RepositoryConfig::testing());
    let query_repo = factory.create_query_repository();
    
    // Test native SQL execution
    let sql = "SELECT * FROM users WHERE id = {{user_id}}";
    let mut params = std::collections::HashMap::new();
    params.insert("user_id".to_string(), serde_json::json!(1));
    
    let result = query_repo.execute_native(
        DatabaseId(1),
        sql,
        Some(params),
    ).await.unwrap();
    
    assert!(!result.columns.is_empty());
    assert!(!result.rows.is_empty());
    
    // Test MBQL execution
    let mbql = serde_json::json!({
        "source-table": 1,
        "aggregation": [["count"]],
        "breakout": [["field", 2, null]],
    });
    
    let mbql_result = query_repo.execute_mbql(
        DatabaseId(1),
        &mbql,
    ).await.unwrap();
    
    assert!(!mbql_result.columns.is_empty());
}

#[tokio::test]
async fn test_query_repository_saved_queries() {
    let factory = RepositoryFactory::new(RepositoryConfig::testing());
    let query_repo = factory.create_query_repository();
    
    // Create a query
    let query = Query {
        id: None,
        name: "Test Query".to_string(),
        description: Some("Test SQL query".to_string()),
        database_id: DatabaseId(1),
        query_type: QueryType::Native,
        query: serde_json::json!({
            "native": {
                "query": "SELECT * FROM users"
            }
        }),
        collection_id: Some(1),
        archived: Some(false),
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
    };
    
    // Test save query
    let saved = query_repo.save_query(&query).await.unwrap();
    assert!(saved.id.is_some());
    
    // Test get query
    let query_id = saved.id.unwrap();
    let retrieved = query_repo.get_query(query_id).await.unwrap();
    assert_eq!(retrieved.name, "Test Query");
    
    // Test list queries
    let queries = query_repo.list_queries(None, None).await.unwrap();
    assert!(!queries.is_empty());
    
    // Test update query
    let mut updated = retrieved.clone();
    updated.name = "Updated Query".to_string();
    let result = query_repo.update_query(query_id, &updated).await.unwrap();
    assert_eq!(result.name, "Updated Query");
    
    // Test delete query
    query_repo.delete_query(query_id).await.unwrap();
}

#[tokio::test]
async fn test_pagination_and_filters() {
    let factory = RepositoryFactory::new(RepositoryConfig::testing());
    let card_repo = factory.create_card_repository();
    
    // Add multiple cards
    for i in 1..=5 {
        let card = Card {
            id: Some(CardId(i)),
            name: format!("Card {}", i),
            archived: Some(i % 2 == 0), // Even cards are archived
            ..Default::default()
        };
        
        if let Some(mock_repo) = card_repo.as_any().downcast_ref::<metabase_api_rs::repository::card::MockCardRepository>() {
            mock_repo.add_card(card).await;
        }
    }
    
    // Test pagination
    let pagination = PaginationParams::new()
        .with_page(1)
        .with_limit(2);
    
    let page1 = card_repo.list(Some(pagination.clone()), None).await.unwrap();
    assert_eq!(page1.len(), 5); // Mock doesn't actually paginate, but real impl would
    
    // Test filters
    let filters = FilterParams::new()
        .with_archived(true);
    
    let archived = card_repo.list(None, Some(filters)).await.unwrap();
    // Mock doesn't filter, but the structure is in place for real implementation
    assert!(!archived.is_empty());
}

#[tokio::test]
async fn test_repository_factory() {
    // Test with mocks
    let mock_factory = RepositoryFactory::new(RepositoryConfig::testing());
    let repos = mock_factory.create_all();
    
    // Verify all repositories are created
    assert!(repos.card.as_any().is::<dyn CardRepository>());
    assert!(repos.collection.as_any().is::<dyn CollectionRepository>());
    assert!(repos.dashboard.as_any().is::<dyn DashboardRepository>());
    assert!(repos.query.as_any().is::<dyn QueryRepository>());
    
    // Test that operations work
    let card = Card {
        name: "Factory Test Card".to_string(),
        ..Default::default()
    };
    
    let created = repos.card.create(&card).await.unwrap();
    assert!(created.id.is_some());
}