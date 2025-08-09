//! Tests for Card API operations

use metabase_api_rs::core::models::{common::CardId, CardBuilder, CardType};
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
async fn test_get_card() {
    let (client, mut server) = setup_test_client().await;

    // Mock the get card endpoint
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

    let card = client.get_card(1).await;
    assert!(card.is_ok());
    let card = card.unwrap();
    assert_eq!(card.id(), Some(CardId(1)));
    assert_eq!(card.name(), "Test Card");
}

#[tokio::test]
async fn test_list_cards() {
    let (client, mut server) = setup_test_client().await;

    // Mock the list cards endpoint
    let _m = server
        .mock("GET", "/api/card")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!([
                {
                    "id": 1,
                    "name": "Card 1",
                    "type": "question",
                    "display": "table",
                    "dataset_query": {
                        "database": 2,
                        "type": "query",
                        "query": {}
                    }
                },
                {
                    "id": 2,
                    "name": "Card 2",
                    "type": "model",
                    "display": "bar",
                    "dataset_query": {
                        "database": 2,
                        "type": "query",
                        "query": {}
                    }
                }
            ])
            .to_string(),
        )
        .create();

    let cards = client.list_cards(None).await;
    assert!(cards.is_ok());
    let cards = cards.unwrap();
    assert_eq!(cards.len(), 2);
    assert_eq!(cards[0].name(), "Card 1");
    assert_eq!(cards[1].name(), "Card 2");
}

#[tokio::test]
async fn test_create_card() {
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

    // Mock the create card endpoint
    let _m = server
        .mock("POST", "/api/card")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": 3,
                "name": "New Card",
                "type": "question",
                "display": "line",
                "dataset_query": {
                    "database": 2,
                    "type": "query",
                    "query": {}
                },
                "created_at": "2023-08-08T10:00:00Z",
                "updated_at": "2023-08-08T10:00:00Z"
            })
            .to_string(),
        )
        .create();

    let new_card = CardBuilder::new(Some(CardId(0)), "New Card".to_string(), CardType::Question)
        .display("line")
        .dataset_query(json!({
            "database": 2,
            "type": "query",
            "query": {}
        }))
        .build();

    let created = client.create_card(new_card).await;
    assert!(created.is_ok());
    let created = created.unwrap();
    assert_eq!(created.id(), Some(CardId(3)));
    assert_eq!(created.name(), "New Card");
}

#[tokio::test]
async fn test_update_card() {
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

    // Mock the update card endpoint
    let _m = server
        .mock("PUT", "/api/card/1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "id": 1,
                "name": "Updated Card",
                "type": "question",
                "display": "table",
                "dataset_query": {
                    "database": 2,
                    "type": "query",
                    "query": {}
                },
                "updated_at": "2023-08-08T11:00:00Z"
            })
            .to_string(),
        )
        .create();

    let updates = json!({
        "name": "Updated Card"
    });

    let updated = client.update_card(1, updates).await;
    assert!(updated.is_ok());
    let updated = updated.unwrap();
    assert_eq!(updated.name(), "Updated Card");
}

#[tokio::test]
async fn test_delete_card() {
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

    // Mock the delete card endpoint
    let _m = server
        .mock("DELETE", "/api/card/1")
        .with_status(204)
        .create();

    let result = client.delete_card(1).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_card_not_found() {
    let (client, mut server) = setup_test_client().await;

    // Mock 404 response
    let _m = server
        .mock("GET", "/api/card/999")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "message": "Card not found"
            })
            .to_string(),
        )
        .create();

    let result = client.get_card(999).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_card_unauthorized() {
    let (client, mut server) = setup_test_client().await;

    // Mock 401 response (not authenticated)
    let _m = server
        .mock("POST", "/api/card")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "message": "Unauthorized"
            })
            .to_string(),
        )
        .create();

    let new_card = CardBuilder::new(Some(CardId(0)), "Test Card".to_string(), CardType::Question)
        .display("table")
        .dataset_query(json!({
            "database": 2,
            "type": "query",
            "query": {}
        }))
        .build();

    let result = client.create_card(new_card).await;
    assert!(result.is_err());
}
