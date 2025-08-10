//! Minimal integration tests for core functionality

use metabase_api_rs::{ClientBuilder, MetabaseClient};
use mockito::{Mock, ServerGuard};
use serde_json::json;

async fn setup_test_client() -> (MetabaseClient, ServerGuard) {
    let server = mockito::Server::new_async().await;
    let url = server.url();
    let client = ClientBuilder::new(&url)
        .build()
        .expect("Failed to create client");
    (client, server)
}

fn mock_auth(server: &mut ServerGuard) -> Mock {
    server
        .mock("POST", "/api/session")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "mock-session-id"}).to_string())
        .create()
}

#[tokio::test]
async fn test_authentication() {
    let (mut client, mut server) = setup_test_client().await;
    let _auth_mock = mock_auth(&mut server);

    let result = client
        .authenticate(metabase_api_rs::api::Credentials::email_password(
            "test@example.com",
            "password",
        ))
        .await;

    assert!(result.is_ok());
    assert!(client.is_authenticated());
}

#[tokio::test]
async fn test_card_operations() {
    let (client, mut server) = setup_test_client().await;

    // Mock GET card
    let _m = server
        .mock("GET", "/api/card/1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": 1,
                "name": "Test Card",
                "type": "question",
                "display": "table",
                "dataset_query": {
                    "database": 2,
                    "type": "query",
                    "query": {}
                },
                "collection_id": null,
                "created_at": "2023-08-08T10:00:00Z",
                "updated_at": "2023-08-08T10:00:00Z"
            })
            .to_string(),
        )
        .create();

    let result = client.get_card(1).await;
    assert!(result.is_ok());

    let card = result.unwrap();
    assert_eq!(
        card.id,
        Some(metabase_api_rs::core::models::common::CardId(1))
    );
    assert_eq!(card.name, "Test Card");
}

#[tokio::test]
async fn test_query_execution() {
    let (mut client, mut server) = setup_test_client().await;

    // Authenticate first
    let _auth_mock = mock_auth(&mut server);
    client
        .authenticate(metabase_api_rs::api::Credentials::email_password(
            "test@example.com",
            "password",
        ))
        .await
        .expect("Failed to authenticate");

    // Mock dataset query
    let _m = server
        .mock("POST", "/api/dataset")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "data": {
                    "rows": [[1, "Test"]],
                    "cols": [
                        {"name": "id", "display_name": "ID", "base_type": "type/Integer"},
                        {"name": "name", "display_name": "Name", "base_type": "type/Text"}
                    ]
                },
                "database_id": 1,
                "started_at": "2023-08-08T10:00:00Z",
                "json_query": {},
                "status": "completed",
                "row_count": 1
            })
            .to_string(),
        )
        .create();

    let query = json!({
        "database": 1,
        "type": "query",
        "query": {"source-table": 10}
    });

    let result = client.execute_dataset_query(query).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_error_handling() {
    let (client, mut server) = setup_test_client().await;

    // Mock 404 response
    let _m = server
        .mock("GET", "/api/card/999")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(json!({"message": "Not found"}).to_string())
        .create();

    let result = client.get_card(999).await;
    assert!(result.is_err());
}
