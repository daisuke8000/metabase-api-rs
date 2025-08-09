//! Tests for Dashboard API operations

use metabase_api_rs::core::models::{common::DashboardId, Dashboard, MetabaseId};
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
async fn test_get_dashboard() {
    let (client, mut server) = setup_test_client().await;

    // Mock the get dashboard endpoint
    let _m = server
        .mock("GET", "/api/dashboard/1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": 1,
                "name": "Test Dashboard",
                "description": "A test dashboard",
                "collection_id": null,
                "creator_id": 1,
                "parameters": [],
                "cards": [],
                "archived": false
            })
            .to_string(),
        )
        .create();

    let dashboard = client.get_dashboard(MetabaseId(1)).await;
    assert!(dashboard.is_ok());
    let dashboard = dashboard.unwrap();
    assert_eq!(dashboard.id, Some(DashboardId(1)));
    assert_eq!(dashboard.name, "Test Dashboard");
}

#[tokio::test]
async fn test_list_dashboards() {
    let (client, mut server) = setup_test_client().await;

    // Mock the list dashboards endpoint
    let _m = server
        .mock("GET", "/api/dashboard")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!([
                {
                    "id": 1,
                    "name": "Dashboard 1",
                    "description": null,
                    "collection_id": null,
                    "parameters": [],
                    "cards": [],
                    "archived": false
                },
                {
                    "id": 2,
                    "name": "Dashboard 2",
                    "description": "Second dashboard",
                    "collection_id": 1,
                    "parameters": [],
                    "cards": [],
                    "archived": false
                }
            ])
            .to_string(),
        )
        .create();

    let dashboards = client.list_dashboards(None).await;
    assert!(dashboards.is_ok());
    let dashboards = dashboards.unwrap();
    assert_eq!(dashboards.len(), 2);
    assert_eq!(dashboards[0].name, "Dashboard 1");
    assert_eq!(dashboards[1].name, "Dashboard 2");
}

#[tokio::test]
async fn test_create_dashboard() {
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

    // Mock the create dashboard endpoint
    let _m = server
        .mock("POST", "/api/dashboard")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": 3,
                "name": "New Dashboard",
                "description": "A newly created dashboard",
                "collection_id": null,
                "parameters": [],
                "cards": [],
                "archived": false
            })
            .to_string(),
        )
        .create();

    let new_dashboard = Dashboard::builder("New Dashboard")
        .description("A newly created dashboard")
        .build();

    let created = client.create_dashboard(new_dashboard).await;
    assert!(created.is_ok());
    let created = created.unwrap();
    assert_eq!(created.id, Some(DashboardId(3)));
    assert_eq!(created.name, "New Dashboard");
}

#[tokio::test]
async fn test_update_dashboard() {
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

    // Mock the update dashboard endpoint
    let _m = server
        .mock("PUT", "/api/dashboard/1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": 1,
                "name": "Updated Dashboard",
                "description": "Updated description",
                "collection_id": null,
                "parameters": [],
                "cards": [],
                "archived": false
            })
            .to_string(),
        )
        .create();

    let updates = json!({
        "name": "Updated Dashboard",
        "description": "Updated description"
    });

    let updated = client.update_dashboard(MetabaseId(1), updates).await;
    assert!(updated.is_ok());
    let updated = updated.unwrap();
    assert_eq!(updated.name, "Updated Dashboard");
}

#[tokio::test]
async fn test_delete_dashboard() {
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

    // Mock the delete dashboard endpoint
    let _m = server
        .mock("DELETE", "/api/dashboard/1")
        .with_status(204)
        .create();

    let result = client.delete_dashboard(MetabaseId(1)).await;
    assert!(result.is_ok());
}
