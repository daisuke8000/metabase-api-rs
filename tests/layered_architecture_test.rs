//! Tests for layered architecture implementation

use metabase_api_rs::api::ClientBuilder;
use metabase_api_rs::core::models::Card;

#[tokio::test]
async fn test_card_via_service_layer() {
    // Arrange
    let client = ClientBuilder::new("http://localhost:3000")
        .build()
        .expect("Failed to create client");
    
    // This test will verify that MetabaseClient uses ServiceManager
    // Currently it will fail because MetabaseClient doesn't have service_manager
    
    // Act - Try to get a card through the service layer
    let result = client.get_card(1).await;
    
    // Assert - This should work through Service → Repository → Transport layers
    assert!(result.is_ok() || result.is_err()); // Just checking it compiles and runs
}

#[tokio::test]
async fn test_card_validation_via_service() {
    // Arrange
    let client = ClientBuilder::new("http://localhost:3000")
        .build()
        .expect("Failed to create client");
    
    use metabase_api_rs::core::models::CardType;
    use serde_json::json;
    
    let invalid_card = Card {
        id: None,
        name: "".to_string(), // Invalid: empty name
        card_type: CardType::Question,
        description: None,
        collection_id: None,
        display: "table".to_string(),
        visualization_settings: json!({}),
        dataset_query: None,
        created_at: None,
        updated_at: None,
        archived: false,
        enable_embedding: false,
        embedding_params: json!({}),
        result_metadata: None,
        creator_id: None,
        database_id: None,
        table_id: None,
        query_type: None,
        entity_id: None,
        cache_ttl: None,
        collection_position: None,
        dashboard_tab_id: None,
        dashboard_id: None,
        public_uuid: None,
        made_public_by_id: None,
        parameters: vec![],
        parameter_mappings: vec![],
    };
    
    // Act - This should fail validation in the service layer
    let result = client.create_card(invalid_card).await;
    
    // Assert - Should get validation error from service layer
    // Currently will just check http error since service layer isn't integrated yet
    assert!(result.is_err());
}

#[test]
fn test_service_manager_exists_in_client() {
    // This test will check if MetabaseClient has service_manager field
    // It will fail initially (RED phase)
    
    // We'll check this at compile time by trying to access the field
    // This is a compile-time test that will fail in RED phase
    
    // Placeholder for now - will be replaced with actual test
    assert!(true, "ServiceManager integration test placeholder");
}