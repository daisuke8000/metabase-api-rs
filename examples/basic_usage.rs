//! Basic usage example for metabase-api-rs
//!
//! This example demonstrates the fundamental operations:
//! - Creating a client
//! - Authentication
//! - Basic API calls

use metabase_api_rs::{api::Credentials, ClientBuilder, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables (optional)
    dotenvy::dotenv().ok();

    // Get Metabase URL from environment or use default
    let base_url =
        std::env::var("METABASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    println!("🚀 Connecting to Metabase at: {}", base_url);

    // Create a client with builder pattern
    let mut client = ClientBuilder::new(&base_url)
        .timeout(Duration::from_secs(30))
        .user_agent("metabase-api-rs-example/0.1.0")
        .build()?;

    println!("✅ Client created successfully");

    // Authenticate using email and password
    let email = std::env::var("METABASE_EMAIL").unwrap_or_else(|_| "admin@example.com".to_string());
    let password = std::env::var("METABASE_PASSWORD").unwrap_or_else(|_| "password123".to_string());

    println!("🔐 Authenticating as: {}", email);

    client
        .authenticate(Credentials::email_password(email, password))
        .await?;

    println!("✅ Authentication successful!");

    // Get current user information
    let current_user = client.get_current_user().await?;
    println!("\n👤 Current User:");
    println!("  - ID: {}", current_user.id);
    println!("  - Email: {}", current_user.email);
    println!(
        "  - Name: {} {}",
        current_user.first_name, current_user.last_name
    );
    println!("  - Active: {}", current_user.is_active);
    println!("  - Superuser: {}", current_user.is_superuser);

    // Note: list_databases is not implemented yet
    println!("\n💾 Available Databases:");
    println!("  (Database listing not yet implemented)");

    // List collections
    println!("\n📁 Collections:");
    let collections = client.list_collections().await?;
    for collection in collections.iter().take(5) {
        // Show first 5
        println!(
            "  - [{:?}] {} ({})",
            collection.id(),
            collection.name(),
            collection.slug().unwrap_or("no-slug")
        );
    }

    // List cards (questions/queries)
    println!("\n📊 Cards (Questions):");
    let cards = client.list_cards(None).await?;
    for card in cards.iter().take(5) {
        // Show first 5
        println!("  - [{:?}] {}", card.id, card.name);
        if let Some(desc) = &card.description {
            println!("    Description: {}", desc);
        }
    }

    // Logout
    println!("\n🔚 Logging out...");
    client.logout().await?;
    println!("✅ Logged out successfully");

    println!("\n🎉 Basic usage example completed successfully!");

    Ok(())
}
