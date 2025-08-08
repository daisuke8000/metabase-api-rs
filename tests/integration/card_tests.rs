//! Card API integration tests

use metabase_api_rs::core::models::{CreateCardRequest, UpdateCardRequest, ListCardsParams};
use serde_json::json;
use crate::integration::setup::{get_test_client, create_test_data, cleanup_test_data};

#[tokio::test]
async fn test_card_lifecycle() {
    let client = get_test_client().await;
    
    // Create a card
    let create_request = CreateCardRequest {
        name: "Integration Test Card".to_string(),
        dataset_query: json!({
            "database": 1,
            "type": "native",
            "native": {
                "query": "SELECT COUNT(*) as total FROM customers"
            }
        }),
        display: "scalar".to_string(),
        visualization_settings: json!({}),
        description: Some("Test card for integration testing".to_string()),
        collection_id: None,
        collection_position: None,
        result_metadata: None,
    };
    
    let card = client.create_card(create_request).await;
    assert!(card.is_ok(), "Card creation should succeed");
    let card = card.unwrap();
    assert_eq!(card.name, "Integration Test Card");
    
    // Get the card
    let fetched_card = client.get_card(card.id.0).await;
    assert!(fetched_card.is_ok(), "Should be able to fetch card");
    assert_eq!(fetched_card.unwrap().id, card.id);
    
    // Update the card
    let update_request = UpdateCardRequest {
        name: Some("Updated Integration Test Card".to_string()),
        description: Some("Updated description".to_string()),
        ..Default::default()
    };
    
    let updated_card = client.update_card(card.id.0, update_request).await;
    assert!(updated_card.is_ok(), "Card update should succeed");
    assert_eq!(updated_card.unwrap().name, "Updated Integration Test Card");
    
    // List cards
    let cards = client.list_cards(ListCardsParams::default()).await;
    assert!(cards.is_ok(), "Should be able to list cards");
    assert!(cards.unwrap().iter().any(|c| c.id == card.id));
    
    // Delete the card
    let delete_result = client.delete_card(card.id.0).await;
    assert!(delete_result.is_ok(), "Card deletion should succeed");
    
    // Verify deletion
    let deleted_card = client.get_card(card.id.0).await;
    assert!(deleted_card.is_err(), "Card should not exist after deletion");
}

#[tokio::test]
async fn test_execute_card_query() {
    let client = get_test_client().await;
    let test_data = create_test_data(&*client).await.expect("Failed to create test data");
    
    // Execute the card query
    let result = client.execute_card_query(test_data.card_id, None).await;
    
    assert!(result.is_ok(), "Card query execution should succeed");
    let query_result = result.unwrap();
    assert_eq!(query_result.status, metabase_api_rs::core::models::QueryStatus::Completed);
    assert!(query_result.data.rows.len() > 0, "Should return some data");
    
    // Cleanup
    cleanup_test_data(&*client, &test_data).await.ok();
}

#[tokio::test] 
async fn test_export_card_query() {
    let client = get_test_client().await;
    let test_data = create_test_data(&*client).await.expect("Failed to create test data");
    
    // Export as CSV
    let csv_result = client
        .export_card_query(
            test_data.card_id,
            metabase_api_rs::core::models::ExportFormat::Csv,
            None,
        )
        .await;
    
    assert!(csv_result.is_ok(), "CSV export should succeed");
    let csv_data = csv_result.unwrap();
    assert!(csv_data.len() > 0, "CSV data should not be empty");
    
    // Verify it's valid CSV
    let csv_str = String::from_utf8(csv_data);
    assert!(csv_str.is_ok(), "Should be valid UTF-8");
    assert!(csv_str.unwrap().contains(","), "CSV should contain commas");
    
    // Cleanup
    cleanup_test_data(&*client, &test_data).await.ok();
}