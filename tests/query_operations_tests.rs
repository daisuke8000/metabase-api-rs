//! Tests for Query execution operations

use metabase_api_rs::core::models::{DatasetQuery, MetabaseId, NativeQuery};
use metabase_api_rs::{ClientBuilder, MetabaseClient};
use mockito::{Mock, ServerGuard};
use serde_json::json;

/// Helper to create a test client and server
async fn setup_test_client() -> (MetabaseClient, ServerGuard) {
    let server = mockito::Server::new_async().await;
    let url = server.url();
    let client = ClientBuilder::new(&url)
        .build()
        .expect("Failed to create client");
    (client, server)
}

/// Create a mock authentication endpoint
fn mock_auth(server: &mut ServerGuard) -> Mock {
    server
        .mock("POST", "/api/session")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": "mock-session-id"
            })
            .to_string(),
        )
        .create()
}

#[tokio::test]
async fn test_execute_query() {
    let (mut client, mut server) = setup_test_client().await;

    // Authenticate first
    let _auth_mock = mock_auth(&mut server);
    client
        .authenticate(metabase_api_rs::api::Credentials::EmailPassword {
            email: "test@example.com".to_string(),
            password: "password".to_string(),
        })
        .await
        .expect("Failed to authenticate");

    // Mock the query execution endpoint
    let _m = server
        .mock("POST", "/api/dataset")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "data": {
                    "cols": [
                        {
                            "name": "id",
                            "display_name": "ID",
                            "base_type": "type/Integer"
                        },
                        {
                            "name": "name",
                            "display_name": "Name",
                            "base_type": "type/Text"
                        }
                    ],
                    "rows": [
                        [1, "John"],
                        [2, "Jane"]
                    ]
                },
                "database_id": 1,
                "started_at": "2023-08-08T10:00:00Z",
                "json_query": {},
                "status": "completed",
                "row_count": 2,
                "running_time": 150
            })
            .to_string(),
        )
        .create();

    let query = DatasetQuery::builder(MetabaseId(1))
        .query_type("query")
        .query(json!({
            "source-table": 1,
            "limit": 10
        }))
        .build();

    let result = client.execute_query(query).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.row_count, Some(2));
    assert_eq!(result.data.rows.len(), 2);
}

#[tokio::test]
async fn test_execute_native_query() {
    let (mut client, mut server) = setup_test_client().await;

    // Authenticate first
    let _auth_mock = mock_auth(&mut server);
    client
        .authenticate(metabase_api_rs::api::Credentials::EmailPassword {
            email: "test@example.com".to_string(),
            password: "password".to_string(),
        })
        .await
        .expect("Failed to authenticate");

    // Mock the native query execution endpoint
    let _m = server
        .mock("POST", "/api/dataset")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "data": {
                    "cols": [
                        {
                            "name": "count",
                            "display_name": "Count",
                            "base_type": "type/Integer"
                        }
                    ],
                    "rows": [
                        [42]
                    ]
                },
                "database_id": 1,
                "started_at": "2023-08-08T10:00:00Z",
                "json_query": {},
                "status": "completed",
                "row_count": 1,
                "running_time": 50
            })
            .to_string(),
        )
        .create();

    let native_query = NativeQuery {
        query: "SELECT COUNT(*) as count FROM users".to_string(),
        template_tags: vec![],
        collection: None,
    };

    let result = client
        .execute_native_query(MetabaseId(1), native_query)
        .await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.row_count, Some(1));
    assert_eq!(result.data.rows.len(), 1);
}

#[tokio::test]
async fn test_execute_query_unauthorized() {
    let (client, mut server) = setup_test_client().await;

    // Mock 401 response (not authenticated)
    let _m = server
        .mock("POST", "/api/dataset")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "message": "Unauthorized"
            })
            .to_string(),
        )
        .create();

    let query = DatasetQuery::builder(MetabaseId(1))
        .query_type("query")
        .query(json!({}))
        .build();

    let result = client.execute_query(query).await;
    assert!(result.is_err());
}
