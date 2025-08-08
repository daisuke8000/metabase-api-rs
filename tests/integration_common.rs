//! Common utilities for integration tests
//!
//! This module provides shared functionality for integration tests.

use std::env;
use std::time::Duration;
use tokio::time::sleep;

/// Load test environment configuration
#[allow(dead_code)]
pub fn load_test_env() {
    dotenvy::from_filename(".env.test").ok();
}

/// Get the Metabase URL from environment or use default
pub fn get_metabase_url() -> String {
    env::var("METABASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
}

/// Get test email from environment
pub fn get_test_email() -> String {
    env::var("METABASE_EMAIL").unwrap_or_else(|_| "test-admin@metabase-test.local".to_string())
}

/// Get test password from environment
pub fn get_test_password() -> String {
    env::var("METABASE_PASSWORD").unwrap_or_else(|_| "TestPassword123!".to_string())
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
                    println!(
                        "Waiting for Metabase... (attempt {}/{})",
                        attempt, max_attempts
                    );
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    Err("Metabase failed to start in time".to_string())
}
