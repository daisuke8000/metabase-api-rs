//! Tests for Extended API operations

use metabase_api_rs::core::models::{ExportFormat, MetabaseId, QueryStatus};
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

// ==================== Card Query Execution Tests ====================

#[tokio::test]
async fn test_execute_card_query() {
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

    // Mock GET request for card existence check (ServiceManager)
    let _get_mock = server
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

    // Mock the execute card query endpoint
    let _m = server
        .mock("POST", "/api/card/1/query")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "data": {
                    "rows": [[1, "Test"], [2, "Data"]],
                    "cols": [
                        {"name": "id", "display_name": "ID", "base_type": "type/Integer"},
                        {"name": "name", "display_name": "Name", "base_type": "type/Text"}
                    ]
                },
                "database_id": 1,
                "started_at": "2023-08-08T10:00:00Z",
                "json_query": {},
                "status": "completed",
                "row_count": 2
            })
            .to_string(),
        )
        .create();

    let result = client.execute_card_query(1, None).await;
    assert!(
        result.is_ok(),
        "Execute card query failed: {:?}",
        result.err()
    );
    let query_result = result.unwrap();
    assert_eq!(query_result.status, QueryStatus::Completed);
}

#[tokio::test]
async fn test_export_card_query_csv() {
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

    // Mock GET request for card existence check (ServiceManager)
    let _get_mock = server
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

    // Mock the export card query endpoint
    let _m = server
        .mock("POST", "/api/card/1/query/csv")
        .with_status(200)
        .with_header("content-type", "text/csv")
        .with_body("id,name\n1,Test\n2,Data")
        .create();

    let result = client.export_card_query(1, ExportFormat::Csv, None).await;
    assert!(
        result.is_ok(),
        "Export card query failed: {:?}",
        result.err()
    );
    let csv_data = result.unwrap();
    assert!(!csv_data.is_empty());
    let csv_string = String::from_utf8(csv_data).unwrap();
    assert!(csv_string.contains("id,name"));
}

#[tokio::test]
async fn test_execute_card_pivot_query() {
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

    // Mock GET request for card existence check (ServiceManager)
    let _get_mock = server
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

    // Mock the pivot query endpoint
    let _m = server
        .mock("POST", "/api/card/pivot/1/query")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "data": {
                    "rows": [[1, 100], [2, 200]],
                    "cols": [
                        {"name": "category", "display_name": "Category", "base_type": "type/Integer"},
                        {"name": "total", "display_name": "Total", "base_type": "type/Integer"}
                    ]
                },
                "database_id": 1,
                "started_at": "2023-08-08T10:00:00Z",
                "json_query": {},
                "status": "completed",
                "row_count": 2
            })
            .to_string(),
        )
        .create();

    let result = client.execute_card_pivot_query(1, None).await;
    assert!(result.is_ok());
}

// ==================== Database Metadata Tests ====================

#[tokio::test]
async fn test_get_database_metadata() {
    let (client, mut server) = setup_test_client().await;

    // Mock the get database metadata endpoint
    let _m = server
        .mock("GET", "/api/database/1/metadata")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": 1,
                "name": "Test Database",
                "engine": "postgres",
                "tables": [
                    {
                        "id": 10,
                        "name": "users",
                        "schema": "public",
                        "display_name": "Users",
                        "fields": [
                            {
                                "id": 100,
                                "name": "id",
                                "display_name": "ID",
                                "database_type": "INTEGER",
                                "base_type": "type/Integer",
                                "is_pk": true,
                                "position": 0,
                                "active": true
                            },
                            {
                                "id": 101,
                                "name": "email",
                                "display_name": "Email",
                                "database_type": "VARCHAR(255)",
                                "base_type": "type/Text",
                                "semantic_type": "type/Email",
                                "is_pk": false,
                                "position": 1,
                                "active": true
                            }
                        ],
                        "active": true
                    }
                ],
                "features": ["basic-aggregations", "foreign-keys"]
            })
            .to_string(),
        )
        .create();

    let result = client.get_database_metadata(MetabaseId(1)).await;
    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.name, "Test Database");
    assert_eq!(metadata.tables.len(), 1);
    assert_eq!(metadata.tables[0].fields.len(), 2);
}

#[tokio::test]
async fn test_sync_database_schema() {
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

    // Mock the sync database schema endpoint
    let _m = server
        .mock("POST", "/api/database/1/sync_schema")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": "sync-task-123",
                "status": "started",
                "message": "Database sync initiated"
            })
            .to_string(),
        )
        .create();

    let result = client.sync_database_schema(MetabaseId(1)).await;
    assert!(result.is_ok());
    let sync_result = result.unwrap();
    assert_eq!(sync_result.status, "started");
}

#[tokio::test]
async fn test_get_database_fields() {
    let (client, mut server) = setup_test_client().await;

    // Mock the get database fields endpoint
    let _m = server
        .mock("GET", "/api/database/1/fields")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!([
                {
                    "id": 100,
                    "table_id": 10,
                    "name": "id",
                    "display_name": "ID",
                    "database_type": "INTEGER",
                    "base_type": "type/Integer",
                    "is_pk": true,
                    "position": 0,
                    "active": true
                },
                {
                    "id": 101,
                    "table_id": 10,
                    "name": "email",
                    "display_name": "Email",
                    "database_type": "VARCHAR(255)",
                    "base_type": "type/Text",
                    "semantic_type": "type/Email",
                    "is_pk": false,
                    "position": 1,
                    "active": true
                }
            ])
            .to_string(),
        )
        .create();

    let result = client.get_database_fields(MetabaseId(1)).await;
    assert!(result.is_ok());
    let fields = result.unwrap();
    assert_eq!(fields.len(), 2);
}

#[tokio::test]
async fn test_get_database_schemas() {
    let (client, mut server) = setup_test_client().await;

    // Mock the get database schemas endpoint
    let _m = server
        .mock("GET", "/api/database/1/schemas")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!(["public", "analytics", "staging"]).to_string())
        .create();

    let result = client.get_database_schemas(MetabaseId(1)).await;
    assert!(result.is_ok());
    let schemas = result.unwrap();
    assert_eq!(schemas.len(), 3);
    assert!(schemas.contains(&"public".to_string()));
}

// ==================== Dataset Operations Tests ====================

#[tokio::test]
async fn test_execute_dataset_query() {
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

    // Mock the dataset query endpoint
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
                "status": "completed"
            })
            .to_string(),
        )
        .create();

    let query = json!({
        "database": 1,
        "type": "query",
        "query": {
            "source-table": 10
        }
    });

    let result = client.execute_dataset_query(query).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_execute_dataset_pivot() {
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

    // Mock the dataset pivot endpoint
    let _m = server
        .mock("POST", "/api/dataset/pivot")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "data": {
                    "rows": [[100, 200]],
                    "cols": [
                        {"name": "Q1", "display_name": "Q1", "base_type": "type/Integer"},
                        {"name": "Q2", "display_name": "Q2", "base_type": "type/Integer"}
                    ]
                },
                "database_id": 1,
                "started_at": "2023-08-08T10:00:00Z",
                "json_query": {},
                "status": "completed"
            })
            .to_string(),
        )
        .create();

    let query = json!({
        "database": 1,
        "type": "query",
        "query": {
            "source-table": 10,
            "aggregation": [["count"]],
            "breakout": [["field", 101, null]]
        }
    });

    let result = client.execute_dataset_pivot(query).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_export_dataset_xlsx() {
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

    // Mock the export dataset endpoint
    let _m = server
        .mock("POST", "/api/dataset/xlsx")
        .with_status(200)
        .with_header(
            "content-type",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        )
        .with_body(vec![0x50, 0x4B]) // Excel file magic bytes
        .create();

    let query = json!({
        "database": 1,
        "type": "query",
        "query": {
            "source-table": 10
        }
    });

    let result = client.export_dataset(ExportFormat::Xlsx, query).await;
    assert!(result.is_ok());
    let xlsx_data = result.unwrap();
    assert!(!xlsx_data.is_empty());
}

// ==================== Error Handling Tests ====================

#[tokio::test]
async fn test_execute_card_query_unauthorized() {
    let (client, mut server) = setup_test_client().await;

    // Mock 401 response
    let _m = server
        .mock("POST", "/api/card/1/query")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "message": "Unauthorized"
            })
            .to_string(),
        )
        .create();

    let result = client.execute_card_query(1, None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_database_metadata_not_found() {
    let (client, mut server) = setup_test_client().await;

    // Mock 404 response
    let _m = server
        .mock("GET", "/api/database/999/metadata")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "message": "Database not found"
            })
            .to_string(),
        )
        .create();

    let result = client.get_database_metadata(MetabaseId(999)).await;
    assert!(result.is_err());
}
