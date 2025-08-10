//! Tests for Collection API operations

use metabase_api_rs::core::models::{common::CollectionId, CollectionBuilder, MetabaseId};
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
async fn test_get_collection() {
    let (client, mut server) = setup_test_client().await;

    // Mock the get collection endpoint
    let _m = server
        .mock("GET", "/api/collection/1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": 1,
                "name": "Test Collection",
                "description": "A test collection",
                "parent_id": null,
                "location": "/",
                "namespace": null,
                "archived": false
            })
            .to_string(),
        )
        .create();

    let collection = client.get_collection(MetabaseId(1)).await;
    assert!(collection.is_ok());
    let collection = collection.unwrap();
    assert_eq!(collection.id(), Some(CollectionId(1)));
    assert_eq!(collection.name(), "Test Collection");
}

#[tokio::test]
async fn test_list_collections() {
    let (client, mut server) = setup_test_client().await;

    // Mock the list collections endpoint
    let _m = server
        .mock("GET", "/api/collection")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!([
                {
                    "id": 1,
                    "name": "Collection 1",
                    "description": null,
                    "parent_id": null,
                    "location": "/",
                    "namespace": null,
                    "archived": false
                },
                {
                    "id": 2,
                    "name": "Collection 2",
                    "description": "Second collection",
                    "parent_id": 1,
                    "location": "/1/",
                    "namespace": null,
                    "archived": false
                }
            ])
            .to_string(),
        )
        .create();

    let collections = client.list_collections().await;
    assert!(collections.is_ok());
    let collections = collections.unwrap();
    assert_eq!(collections.len(), 2);
    assert_eq!(collections[0].name(), "Collection 1");
    assert_eq!(collections[1].name(), "Collection 2");
}

#[tokio::test]
async fn test_create_collection() {
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

    // Mock the create collection endpoint
    let _m = server
        .mock("POST", "/api/collection")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": 3,
                "name": "New Collection",
                "description": "A newly created collection",
                "parent_id": null,
                "location": "/",
                "namespace": null,
                "archived": false
            })
            .to_string(),
        )
        .create();

    let new_collection = CollectionBuilder::new_collection("New Collection")
        .description("A newly created collection")
        .build();

    let created = client.create_collection(new_collection).await;
    assert!(created.is_ok());
    let created = created.unwrap();
    assert_eq!(created.id(), Some(CollectionId(3)));
    assert_eq!(created.name(), "New Collection");
}

#[tokio::test]
async fn test_update_collection() {
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

    // Mock GET request for existing collection (API layer fetches for merge)
    let _get_mock = server
        .mock("GET", "/api/collection/1")
        .expect(2) // Expect 2 GET requests (API + Service layer)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": 1,
                "name": "Original Collection",
                "description": "Original description",
                "parent_id": null,
                "location": "/",
                "namespace": null,
                "archived": false
            })
            .to_string(),
        )
        .create();

    // Mock the update collection endpoint
    let _put_mock = server
        .mock("PUT", "/api/collection/1")
        .match_body(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": 1,
                "name": "Updated Collection",
                "description": "Updated description",
                "parent_id": null,
                "location": "/",
                "namespace": null,
                "archived": false
            })
            .to_string(),
        )
        .create();

    let updates = json!({
        "name": "Updated Collection",
        "description": "Updated description"
    });

    let updated = client.update_collection(MetabaseId(1), updates).await;
    assert!(updated.is_ok());
    let updated = updated.unwrap();
    assert_eq!(updated.name(), "Updated Collection");
}

#[tokio::test]
async fn test_delete_collection() {
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

    // Mock the archive collection endpoint and GET for response
    let _archive_mock = server
        .mock("PUT", "/api/collection/1")
        .match_body(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({}).to_string()) // Archive returns empty response
        .create();

    // Mock GET for the archived collection response
    let _get_mock = server
        .mock("GET", "/api/collection/1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": 1,
                "name": "Archived Collection",
                "description": null,
                "parent_id": null,
                "location": "/",
                "namespace": null,
                "archived": true
            })
            .to_string(),
        )
        .create();

    let result = client.archive_collection(MetabaseId(1)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_collection_not_found() {
    let (client, mut server) = setup_test_client().await;

    // Mock 404 response
    let _m = server
        .mock("GET", "/api/collection/999")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "message": "Collection not found"
            })
            .to_string(),
        )
        .create();

    let result = client.get_collection(MetabaseId(999)).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_collection_unauthorized() {
    let (client, mut server) = setup_test_client().await;

    // Mock 401 response (not authenticated)
    let _m = server
        .mock("POST", "/api/collection")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "message": "Unauthorized"
            })
            .to_string(),
        )
        .create();

    let new_collection = CollectionBuilder::new_collection("Test Collection").build();

    let result = client.create_collection(new_collection).await;
    assert!(result.is_err());
}
