//! Integration test module for metabase-api-rs
//!
//! These tests run against a real Metabase instance using Docker.

use std::env;
use std::time::Duration;
use tokio::time::sleep;

pub mod setup;
pub mod auth_tests;
pub mod card_tests;
pub mod collection_tests;
pub mod database_tests;
pub mod query_tests;

/// Load test environment configuration
pub fn load_test_env() {
    dotenv::from_filename(".env.test").ok();
}

/// Get the Metabase URL from environment or use default
pub fn get_metabase_url() -> String {
    env::var("METABASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
}

/// Get test email from environment
pub fn get_test_email() -> String {
    env::var("METABASE_EMAIL").unwrap_or_else(|_| "test@example.com".to_string())
}

/// Get test password from environment
pub fn get_test_password() -> String {
    env::var("METABASE_PASSWORD").unwrap_or_else(|_| "test_password123".to_string())
}

/// Wait for Metabase to be ready
pub async fn wait_for_metabase(max_attempts: u32) -> Result<(), String> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/health", get_metabase_url());
    
    for attempt in 1..=max_attempts {
        match client.get(&url).send().await {
            Ok(response) if response.status().is_success() => {
                println!("Metabase is ready after {} attempts", attempt);
                return Ok(());
            }
            _ => {
                if attempt < max_attempts {
                    println!("Waiting for Metabase... (attempt {}/{})", attempt, max_attempts);
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }
    
    Err("Metabase failed to start in time".to_string())
}

/// Setup test user in Metabase
pub async fn setup_test_user() -> Result<(), Box<dyn std::error::Error>> {
    // This would be called once during test setup to create the test user
    // For now, we assume the user is created manually or through docker init
    Ok(())
}

/// Cleanup test data after tests
pub async fn cleanup_test_data() -> Result<(), Box<dyn std::error::Error>> {
    // Clean up any test data created during integration tests
    Ok(())
}