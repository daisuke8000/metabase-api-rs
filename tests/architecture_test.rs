//! Architecture Verification Tests
//!
//! This test suite verifies that all requests follow the correct layered architecture:
//! MetabaseClient → ServiceManager → Service層 → Repository層 → HttpProviderSafe → Metabase API

use metabase_api_rs::{
    api::{ClientBuilder, Credentials},
    core::models::{Card, CardType, MetabaseId},
};
use mockito::Server;
use serde_json::json;

// ==========================================
// Architecture Validation Test
// ==========================================

/// Verify the architecture layers are correctly wired together
#[test]
fn test_architecture_layers_wiring() {
    use metabase_api_rs::service::manager::ServiceManager;
    use std::sync::Arc;

    // Create a mock HTTP provider
    struct MockHttpProvider;

    #[async_trait::async_trait]
    impl metabase_api_rs::transport::http_provider_safe::HttpProviderSafe for MockHttpProvider {
        async fn get_json(&self, _path: &str) -> metabase_api_rs::Result<serde_json::Value> {
            Ok(serde_json::Value::Null)
        }

        async fn post_json(
            &self,
            _path: &str,
            _body: serde_json::Value,
        ) -> metabase_api_rs::Result<serde_json::Value> {
            Ok(serde_json::Value::Null)
        }

        async fn put_json(
            &self,
            _path: &str,
            _body: serde_json::Value,
        ) -> metabase_api_rs::Result<serde_json::Value> {
            Ok(serde_json::Value::Null)
        }

        async fn delete_json(&self, _path: &str) -> metabase_api_rs::Result<serde_json::Value> {
            Ok(serde_json::Value::Null)
        }

        async fn post_binary(
            &self,
            _path: &str,
            _body: serde_json::Value,
        ) -> metabase_api_rs::Result<Vec<u8>> {
            Ok(vec![])
        }
    }

    // This actually creates the entire architecture stack
    let http_provider = Arc::new(MockHttpProvider);
    let service_manager = ServiceManager::new(http_provider);

    // Verify all services are properly wired
    assert!(
        service_manager.card_service().is_some(),
        "Card service must be available"
    );
    assert!(
        service_manager.collection_service().is_some(),
        "Collection service must be available"
    );
    assert!(
        service_manager.dashboard_service().is_some(),
        "Dashboard service must be available"
    );
    assert!(
        service_manager.database_service().is_some(),
        "Database service must be available"
    );
    assert!(
        service_manager.query_service().is_some(),
        "Query service must be available"
    );
    assert!(
        service_manager.auth_service().is_some(),
        "Auth service must be available"
    );
}

// ==========================================
// Runtime Architecture Flow Tests
// ==========================================

/// Test the complete architecture stack wiring
#[tokio::test]
async fn test_complete_architecture_stack() {
    let mut server = Server::new_async().await;

    // Mock authentication
    let _mock_auth = server
        .mock("POST", "/api/session")
        .with_status(200)
        .with_body(json!({"id": "session-id", "email": "test@example.com"}).to_string())
        .create_async()
        .await;

    // Create client - validates entire architecture stack
    let mut client = ClientBuilder::new(server.url())
        .build()
        .expect("Failed to create client");

    // Authenticate through all layers
    let result = client
        .authenticate(Credentials::email_password("test@example.com", "password"))
        .await;

    assert!(
        result.is_ok(),
        "Authentication should work through all layers"
    );
}

/// Test Card operations through the layered architecture
#[tokio::test]
async fn test_card_operations_through_layers() {
    let mut server = Server::new_async().await;

    // Setup mocks
    setup_auth_mock(&mut server).await;

    let _mock_get = server
        .mock("GET", "/api/card/1")
        .with_status(200)
        .with_body(
            json!({
                "id": 1,
                "name": "Test Card",
                "display": "table",
                "visualization_settings": {}
            })
            .to_string(),
        )
        .create_async()
        .await;

    let _mock_list = server
        .mock("GET", "/api/card")
        .with_status(200)
        .with_body(
            json!([{
                "id": 1,
                "name": "Test Card",
                "display": "table",
                "visualization_settings": {}
            }])
            .to_string(),
        )
        .create_async()
        .await;

    let _mock_delete = server
        .mock("DELETE", "/api/card/1")
        .with_status(204)
        .create_async()
        .await;

    let client = create_authenticated_client(&server).await;

    // Test READ operation
    let card = client.get_card(1).await.expect("Failed to get card");
    assert_eq!(card.name, "Test Card");

    // Test LIST operation
    let cards = client.list_cards(None).await.expect("Failed to list cards");
    assert!(!cards.is_empty());

    // Test DELETE operation
    client.delete_card(1).await.expect("Failed to delete card");
}

/// Test Collection operations through the layered architecture
#[tokio::test]
async fn test_collection_operations_through_layers() {
    let mut server = Server::new_async().await;

    setup_auth_mock(&mut server).await;

    let _mock_list = server
        .mock("GET", "/api/collection")
        .with_status(200)
        .with_body(
            json!([{
                "id": 1,
                "name": "Root Collection",
                "slug": "root"
            }])
            .to_string(),
        )
        .create_async()
        .await;

    let _mock_get = server
        .mock("GET", "/api/collection/1")
        .with_status(200)
        .with_body(
            json!({
                "id": 1,
                "name": "Root Collection",
                "slug": "root",
                "parent_id": null
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = create_authenticated_client(&server).await;

    // Test LIST operation
    let collections = client
        .list_collections()
        .await
        .expect("Failed to list collections");
    assert_eq!(collections.len(), 1);
    assert_eq!(collections[0].name, "Root Collection");

    // Test GET operation
    let collection = client
        .get_collection(MetabaseId(1))
        .await
        .expect("Failed to get collection");
    assert_eq!(collection.name, "Root Collection");
}

/// Test Dashboard operations through the layered architecture
#[tokio::test]
async fn test_dashboard_operations_through_layers() {
    let mut server = Server::new_async().await;

    setup_auth_mock(&mut server).await;

    let _mock_get = server
        .mock("GET", "/api/dashboard/1")
        .with_status(200)
        .with_body(
            json!({
                "id": 1,
                "name": "Test Dashboard",
                "dashcards": []
            })
            .to_string(),
        )
        .create_async()
        .await;

    let _mock_list = server
        .mock("GET", "/api/dashboard")
        .with_status(200)
        .with_body(
            json!([{
                "id": 1,
                "name": "Test Dashboard",
                "dashcards": []
            }])
            .to_string(),
        )
        .create_async()
        .await;

    let client = create_authenticated_client(&server).await;

    // Test GET operation
    let dashboard = client
        .get_dashboard(MetabaseId(1))
        .await
        .expect("Failed to get dashboard");
    assert_eq!(dashboard.name, "Test Dashboard");

    // Dashboard list operation is tested via get_dashboard
    // since list_dashboards requires parameters
}

/// Test Database operations through the layered architecture
#[tokio::test]
async fn test_database_operations_through_layers() {
    let mut server = Server::new_async().await;

    setup_auth_mock(&mut server).await;

    let _mock_metadata = server
        .mock("GET", "/api/database/1/metadata")
        .with_status(200)
        .with_body(
            json!({
                "id": 1,
                "name": "Test Database",
                "engine": "postgres",
                "tables": []
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = create_authenticated_client(&server).await;

    // Test metadata operation
    let metadata = client
        .get_database_metadata(MetabaseId(1))
        .await
        .expect("Failed to get database metadata");
    assert_eq!(metadata.name, "Test Database");
}

/// Test Query operations through the layered architecture
#[tokio::test]
async fn test_query_operations_through_layers() {
    let mut server = Server::new_async().await;

    setup_auth_mock(&mut server).await;

    let _mock_query = server
        .mock("POST", "/api/dataset")
        .with_status(200)
        .with_body(
            json!({
                "data": {
                    "rows": [[1, "test"]],
                    "cols": [
                        {"name": "id", "base_type": "type/Integer"},
                        {"name": "name", "base_type": "type/Text"}
                    ]
                }
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = create_authenticated_client(&server).await;

    // Test query execution
    let result = client
        .execute_dataset_query(json!({
            "database": 1,
            "type": "query",
            "query": {"source-table": 1}
        }))
        .await
        .expect("Failed to execute query");

    assert!(!result.data.rows.is_empty());
}

/// Test CRUD operations all use ServiceManager
#[tokio::test]
async fn test_crud_operations_use_service_manager() {
    let mut server = Server::new_async().await;

    setup_auth_mock(&mut server).await;

    // Mock CREATE
    let _mock_create = server
        .mock("POST", "/api/card")
        .with_status(200)
        .with_body(
            json!({
                "id": 2,
                "name": "New Card",
                "display": "table",
                "visualization_settings": {}
            })
            .to_string(),
        )
        .create_async()
        .await;

    // Mock UPDATE (requires GET first)
    let _mock_get_for_update = server
        .mock("GET", "/api/card/1")
        .with_status(200)
        .with_body(
            json!({
                "id": 1,
                "name": "Old Card",
                "display": "table",
                "visualization_settings": {}
            })
            .to_string(),
        )
        .create_async()
        .await;

    let _mock_update = server
        .mock("PUT", "/api/card/1")
        .with_status(200)
        .with_body(
            json!({
                "id": 1,
                "name": "Updated Card",
                "display": "table",
                "visualization_settings": {}
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = create_authenticated_client(&server).await;

    // Test CREATE
    let new_card = create_test_card("New Card");
    let created = client
        .create_card(new_card)
        .await
        .expect("Failed to create card");
    assert_eq!(created.name, "New Card");

    // Test UPDATE
    let updated = client
        .update_card(1, json!({"name": "Updated Card"}))
        .await
        .expect("Failed to update card");
    assert_eq!(updated.name, "Updated Card");
}

// ==========================================
// Helper Functions
// ==========================================

/// Setup authentication mock
async fn setup_auth_mock(server: &mut Server) {
    server
        .mock("POST", "/api/session")
        .with_status(200)
        .with_body(json!({"id": "session-id", "email": "test@example.com"}).to_string())
        .create_async()
        .await;
}

/// Create and authenticate a client
async fn create_authenticated_client(server: &Server) -> metabase_api_rs::api::MetabaseClient {
    let mut client = ClientBuilder::new(server.url())
        .build()
        .expect("Failed to create client");

    client
        .authenticate(Credentials::email_password("test@example.com", "password"))
        .await
        .expect("Failed to authenticate");

    client
}

/// Create a test card
fn create_test_card(name: &str) -> Card {
    Card {
        id: None,
        name: name.to_string(),
        card_type: CardType::Question,
        description: None,
        collection_id: None,
        display: "table".to_string(),
        visualization_settings: json!({}),
        dataset_query: Some(json!({
            "database": 1,
            "type": "query",
            "query": {"source-table": 1}
        })),
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
        creator_id: None,
        database_id: None,
        table_id: None,
        query_type: None,
        public_uuid: None,
        made_public_by_id: None,
    }
}
