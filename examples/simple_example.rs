//! Simple example demonstrating basic metabase-api-rs usage
//!
//! Run with: cargo run --example simple_example

use metabase_api_rs::{api::Credentials, ClientBuilder, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Use environment variables or defaults
    dotenvy::dotenv().ok();

    let base_url =
        std::env::var("METABASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let email = std::env::var("METABASE_EMAIL").unwrap_or_else(|_| "admin@example.com".to_string());
    let password = std::env::var("METABASE_PASSWORD").unwrap_or_else(|_| "password123".to_string());

    println!("Connecting to Metabase at: {}", base_url);

    // Create client
    let mut client = ClientBuilder::new(&base_url)
        .timeout(Duration::from_secs(30))
        .build()?;

    // Authenticate
    println!("Authenticating...");
    client
        .authenticate(Credentials::email_password(email, password))
        .await?;
    println!("Authentication successful!");

    // Get current user
    let user = client.get_current_user().await?;
    println!("Logged in as: {} (ID: {})", user.email, user.id);

    // List collections
    let collections = client.list_collections().await?;
    println!("\nCollections: {} found", collections.len());

    // List cards (with pagination)
    let cards = client.list_cards(None).await?;
    println!("\nCards: {} found", cards.len());

    println!("\nExample completed successfully!");
    Ok(())
}
