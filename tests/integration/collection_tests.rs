//! Collection API integration tests

use metabase_api_rs::core::models::{CreateCollectionRequest, UpdateCollectionRequest};
use crate::integration::setup::get_test_client;

#[tokio::test]
async fn test_collection_lifecycle() {
    let client = get_test_client().await;
    
    // Create a collection
    let create_request = CreateCollectionRequest {
        name: "Test Collection".to_string(),
        color: "#FF5733".to_string(),
        description: Some("Integration test collection".to_string()),
        parent_id: None,
    };
    
    let collection = client.create_collection(create_request).await;
    assert!(collection.is_ok(), "Collection creation should succeed");
    let collection = collection.unwrap();
    assert_eq!(collection.name, "Test Collection");
    
    // Get the collection
    let fetched = client.get_collection(collection.id.0).await;
    assert!(fetched.is_ok(), "Should be able to fetch collection");
    assert_eq!(fetched.unwrap().id, collection.id);
    
    // Update the collection
    let update_request = UpdateCollectionRequest {
        name: Some("Updated Test Collection".to_string()),
        description: Some("Updated description".to_string()),
        color: Some("#00FF00".to_string()),
        ..Default::default()
    };
    
    let updated = client.update_collection(collection.id.0, update_request).await;
    assert!(updated.is_ok(), "Collection update should succeed");
    let updated = updated.unwrap();
    assert_eq!(updated.name, "Updated Test Collection");
    assert_eq!(updated.color, "#00FF00");
    
    // List collections
    let collections = client.list_collections().await;
    assert!(collections.is_ok(), "Should be able to list collections");
    assert!(collections.unwrap().iter().any(|c| c.id == collection.id));
    
    // Delete the collection
    let delete_result = client.delete_collection(collection.id.0).await;
    assert!(delete_result.is_ok(), "Collection deletion should succeed");
    
    // Verify deletion
    let deleted = client.get_collection(collection.id.0).await;
    assert!(deleted.is_err(), "Collection should not exist after deletion");
}

#[tokio::test]
async fn test_nested_collections() {
    let client = get_test_client().await;
    
    // Create parent collection
    let parent = client
        .create_collection(CreateCollectionRequest {
            name: "Parent Collection".to_string(),
            color: "#0000FF".to_string(),
            description: Some("Parent collection".to_string()),
            parent_id: None,
        })
        .await
        .expect("Parent collection creation should succeed");
    
    // Create child collection
    let child = client
        .create_collection(CreateCollectionRequest {
            name: "Child Collection".to_string(),
            color: "#FF00FF".to_string(),
            description: Some("Child collection".to_string()),
            parent_id: Some(parent.id.0),
        })
        .await;
    
    assert!(child.is_ok(), "Child collection creation should succeed");
    let child = child.unwrap();
    assert_eq!(child.parent_id, Some(parent.id.0));
    
    // List collection items
    let items = client.list_collection_items(parent.id.0).await;
    assert!(items.is_ok(), "Should be able to list collection items");
    
    // Cleanup
    client.delete_collection(child.id.0).await.ok();
    client.delete_collection(parent.id.0).await.ok();
}