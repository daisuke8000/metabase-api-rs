//! Smoke tests - Minimum verification for basic functionality

use metabase_api_rs::ClientBuilder;

#[test]
fn test_client_creation() {
    let result = ClientBuilder::new("http://localhost:3000").build();
    assert!(result.is_ok());
}

#[test]
fn test_invalid_url_rejected() {
    let result = ClientBuilder::new("invalid-url").build();
    assert!(result.is_err());
}
