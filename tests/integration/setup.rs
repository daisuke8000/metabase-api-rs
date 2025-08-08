//! Test setup utilities for integration tests

use metabase_api_rs::{ClientBuilder, MetabaseClient};
use std::sync::Arc;
use tokio::sync::OnceCell;

use super::{get_metabase_url, get_test_email, get_test_password, wait_for_metabase};

/// Global test client instance (shared across tests)
static TEST_CLIENT: OnceCell<Arc<MetabaseClient>> = OnceCell::const_new();

/// Get or create a test client
pub async fn get_test_client() -> Arc<MetabaseClient> {
    TEST_CLIENT
        .get_or_init(|| async {
            // Ensure Metabase is ready
            wait_for_metabase(30)
                .await
                .expect("Metabase should be running");

            // Create and authenticate client
            let mut client = ClientBuilder::new(&get_metabase_url())
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to build client");

            client
                .authenticate(metabase_api_rs::api::Credentials::EmailPassword {
                    email: get_test_email(),
                    password: get_test_password(),
                })
                .await
                .expect("Failed to authenticate");

            Arc::new(client)
        })
        .await
        .clone()
}

/// Setup sample database connection in Metabase
pub async fn setup_sample_database(client: &MetabaseClient) -> metabase_api_rs::Result<i64> {
    use serde_json::json;
    
    // This would add the sample database to Metabase
    // For now, we'll assume it's added manually or through setup
    
    // Return a mock database ID for testing
    Ok(1)
}

/// Create test data for integration tests
pub async fn create_test_data(client: &MetabaseClient) -> metabase_api_rs::Result<TestData> {
    use metabase_api_rs::core::models::{CreateCardRequest, CreateCollectionRequest};
    use serde_json::json;

    // Create a test collection
    let collection = client
        .create_collection(CreateCollectionRequest {
            name: "Integration Test Collection".to_string(),
            color: "#509EE3".to_string(),
            description: Some("Collection for integration tests".to_string()),
            parent_id: None,
        })
        .await?;

    // Create a test card
    let card = client
        .create_card(CreateCardRequest {
            name: "Test Query Card".to_string(),
            dataset_query: json!({
                "database": 1,
                "type": "native",
                "native": {
                    "query": "SELECT * FROM customers LIMIT 10"
                }
            }),
            display: "table".to_string(),
            visualization_settings: json!({}),
            description: Some("Test card for integration tests".to_string()),
            collection_id: Some(collection.id.0),
            collection_position: None,
            result_metadata: None,
        })
        .await?;

    Ok(TestData {
        collection_id: collection.id.0,
        card_id: card.id.0,
        database_id: 1,
    })
}

/// Test data container
#[derive(Debug, Clone)]
pub struct TestData {
    pub collection_id: i64,
    pub card_id: i64,
    pub database_id: i64,
}

/// Cleanup test data after tests
pub async fn cleanup_test_data(client: &MetabaseClient, data: &TestData) -> metabase_api_rs::Result<()> {
    // Delete test card
    let _ = client.delete_card(data.card_id).await;
    
    // Delete test collection
    let _ = client.delete_collection(data.collection_id).await;
    
    Ok(())
}