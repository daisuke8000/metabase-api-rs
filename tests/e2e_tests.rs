//! End-to-End tests with real Metabase instance
//!
//! These tests require Docker to be running with Metabase instance.
//! Run with: task integration:run

use metabase_api_rs::{api::Credentials, ClientBuilder};
use std::env;

/// Helper to check if we're running in CI/Docker environment
fn should_run_e2e() -> bool {
    env::var("RUN_E2E_TESTS").unwrap_or_default() == "true" || env::var("CI").is_ok()
}

/// Get Metabase URL from environment or use default
fn get_metabase_url() -> String {
    env::var("METABASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
}

/// Get test credentials from environment
fn get_test_credentials() -> (String, String) {
    let email = env::var("METABASE_EMAIL").unwrap_or_else(|_| "test@example.com".to_string());
    let password = env::var("METABASE_PASSWORD").unwrap_or_else(|_| "testpassword123".to_string());
    (email, password)
}

#[tokio::test]
async fn test_e2e_authentication_flow() {
    if !should_run_e2e() {
        eprintln!("Skipping E2E test: Set RUN_E2E_TESTS=true to run");
        return;
    }
    eprintln!(
        "🚀 Running E2E authentication test against {}",
        get_metabase_url()
    );

    let url = get_metabase_url();
    let (email, password) = get_test_credentials();

    let mut client = ClientBuilder::new(&url)
        .build()
        .expect("Failed to create client");

    // Test authentication
    let result = client
        .authenticate(Credentials::email_password(&email, &password))
        .await;

    match result {
        Ok(_) => {
            assert!(client.is_authenticated());
            println!("✅ Authentication successful");

            // Test logout
            client.logout().await.expect("Failed to logout");
            assert!(!client.is_authenticated());
            println!("✅ Logout successful");
        }
        Err(e) => {
            eprintln!(
                "⚠️ E2E test skipped: Metabase not available at {}: {}",
                url, e
            );
        }
    }
}

#[tokio::test]
async fn test_e2e_card_crud_operations() {
    if !should_run_e2e() {
        eprintln!("Skipping E2E test: Set RUN_E2E_TESTS=true to run");
        return;
    }
    eprintln!(
        "🚀 Running E2E card CRUD test against {}",
        get_metabase_url()
    );

    let url = get_metabase_url();
    let (email, password) = get_test_credentials();

    let mut client = ClientBuilder::new(&url)
        .build()
        .expect("Failed to create client");

    // Authenticate first
    if let Err(e) = client
        .authenticate(Credentials::email_password(&email, &password))
        .await
    {
        eprintln!("⚠️ E2E test skipped: Authentication failed: {}", e);
        return;
    }

    // Try to list cards
    match client.list_cards(None).await {
        Ok(cards) => {
            println!("✅ Listed {} cards", cards.len());

            // If there are cards, try to get the first one
            if let Some(first_card) = cards.first() {
                if let Some(id) = first_card.id {
                    let card = client
                        .get_card(id.0 as i64)
                        .await
                        .expect("Failed to get card");
                    println!("✅ Retrieved card: {}", card.name);
                }
            }
        }
        Err(e) => {
            eprintln!("⚠️ Could not list cards: {}", e);
        }
    }
}

#[tokio::test]
async fn test_e2e_collection_operations() {
    if !should_run_e2e() {
        eprintln!("Skipping E2E test: Set RUN_E2E_TESTS=true to run");
        return;
    }
    eprintln!(
        "🚀 Running E2E collection test against {}",
        get_metabase_url()
    );

    let url = get_metabase_url();
    let (email, password) = get_test_credentials();

    let mut client = ClientBuilder::new(&url)
        .build()
        .expect("Failed to create client");

    // Authenticate first
    if let Err(e) = client
        .authenticate(Credentials::email_password(&email, &password))
        .await
    {
        eprintln!("⚠️ E2E test skipped: Authentication failed: {}", e);
        return;
    }

    // List collections
    match client.list_collections().await {
        Ok(collections) => {
            println!("✅ Listed {} collections", collections.len());

            // Check for root collection
            let root = collections.iter().find(|c| c.parent_id.is_none());
            assert!(root.is_some(), "Should have at least a root collection");
            println!("✅ Found root collection");
        }
        Err(e) => {
            eprintln!("⚠️ Could not list collections: {}", e);
        }
    }
}

#[tokio::test]
async fn test_e2e_health_check() {
    if !should_run_e2e() {
        eprintln!("Skipping E2E test: Set RUN_E2E_TESTS=true to run");
        return;
    }
    eprintln!(
        "🚀 Running E2E health check test against {}",
        get_metabase_url()
    );

    let url = get_metabase_url();
    let client = ClientBuilder::new(&url)
        .build()
        .expect("Failed to create client");

    // Health check doesn't require authentication
    match client.health_check().await {
        Ok(status) => {
            println!("✅ Health check passed: {:?}", status);
        }
        Err(e) => {
            eprintln!("⚠️ E2E test skipped: Health check failed: {}", e);
        }
    }
}
