//! Simplified CRUD operations example for metabase-api-rs
//!
//! This example demonstrates basic Create, Read, Update, and Delete operations

use metabase_api_rs::{api::Credentials, models::MetabaseId, ClientBuilder, Result};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let base_url =
        std::env::var("METABASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    println!("🚀 Simple CRUD Operations Example");
    println!("{}", "=".repeat(50));

    // Create and authenticate client
    let mut client = ClientBuilder::new(&base_url).build()?;

    let email = std::env::var("METABASE_EMAIL").unwrap_or_else(|_| "admin@example.com".to_string());
    let password = std::env::var("METABASE_PASSWORD").unwrap_or_else(|_| "password123".to_string());

    client
        .authenticate(Credentials::email_password(email, password))
        .await?;

    println!("✅ Authenticated successfully");

    // ============================================
    // COLLECTION OPERATIONS
    // ============================================
    println!("\n📁 COLLECTION OPERATIONS");
    println!("{}", "-".repeat(40));

    // LIST Collections
    println!("\n1. Listing collections...");
    let collections = client.list_collections().await?;
    println!("   Found {} collections", collections.len());
    for col in collections.iter().take(3) {
        println!("   - [{:?}] {}", col.id(), col.name());
    }

    // READ specific collection
    if let Some(first_collection) = collections.first() {
        println!("\n2. Reading collection details...");
        let collection = client
            .get_collection(MetabaseId(first_collection.id().unwrap().0.into()))
            .await?;
        println!("   Name: {}", collection.name());
        println!("   ID: {:?}", collection.id());
        if let Some(desc) = collection.description() {
            println!("   Description: {}", desc);
        }
    }

    // ============================================
    // CARD OPERATIONS
    // ============================================
    println!("\n\n📊 CARD OPERATIONS");
    println!("{}", "-".repeat(40));

    // LIST Cards
    println!("\n1. Listing cards...");
    let cards = client.list_cards(None).await?;
    println!("   Found {} cards", cards.len());
    for card in cards.iter().take(3) {
        println!("   - [{:?}] {}", card.id, card.name);
    }

    // READ Card details
    if let Some(first_card) = cards.first() {
        println!("\n2. Reading card details...");
        let card = client.get_card(first_card.id.unwrap().0.into()).await?;
        println!("   Name: {}", card.name);
        println!("   ID: {:?}", card.id);
        if let Some(desc) = &card.description {
            println!("   Description: {}", desc);
        }

        // RUN Card Query
        println!("\n3. Running card query...");
        match client
            .execute_card_query(card.id.unwrap().0.into(), None)
            .await
        {
            Ok(result) => {
                if let Some(row_count) = result.row_count {
                    println!("   Query result rows: {}", row_count);
                }
                if let Some(running_time) = result.running_time {
                    println!("   Execution time: {}ms", running_time);
                }
            }
            Err(e) => {
                println!("   Could not run query: {}", e);
            }
        }
    }

    // ============================================
    // DASHBOARD OPERATIONS
    // ============================================
    println!("\n\n📈 DASHBOARD OPERATIONS");
    println!("{}", "-".repeat(40));

    // LIST Dashboards
    println!("\n1. Listing dashboards...");
    let dashboards = client.list_dashboards(None).await?;
    println!("   Found {} dashboards", dashboards.len());
    for dash in dashboards.iter().take(3) {
        println!("   - [{:?}] {}", dash.id, dash.name);
    }

    // READ Dashboard details
    if let Some(first_dashboard) = dashboards.first() {
        println!("\n2. Reading dashboard details...");
        let dashboard = client
            .get_dashboard(MetabaseId(first_dashboard.id.unwrap().0.into()))
            .await?;
        println!("   Name: {}", dashboard.name);
        println!("   ID: {:?}", dashboard.id);
        if let Some(desc) = &dashboard.description {
            println!("   Description: {}", desc);
        }
    }

    // ============================================
    // UPDATE EXAMPLE (using JSON)
    // ============================================
    println!("\n\n✏️ UPDATE OPERATIONS");
    println!("{}", "-".repeat(40));

    // Example of updating a card (if we have one)
    if let Some(card) = cards.first() {
        println!("\nUpdating card name...");
        let updates = json!({
            "name": format!("{} (Updated)", card.name)
        });

        match client.update_card(card.id.unwrap().0.into(), updates).await {
            Ok(updated_card) => {
                println!("✅ Updated card: {}", updated_card.name);
            }
            Err(e) => {
                println!("⚠️ Could not update card: {}", e);
            }
        }
    }

    println!("\n\n✅ CRUD operations example completed!");

    Ok(())
}
