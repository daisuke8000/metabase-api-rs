use metabase_api_rs::api::ClientBuilder;
use std::time::Duration;

#[test]
fn test_builder_basic_configuration() {
    let client = ClientBuilder::new("https://metabase.example.com")
        .build();
    
    assert!(client.is_ok());
    let client = client.unwrap();
    assert_eq!(client.base_url(), "https://metabase.example.com");
}

#[test]
fn test_builder_custom_timeout() {
    let timeout = Duration::from_secs(60);
    let client = ClientBuilder::new("https://metabase.example.com")
        .timeout(timeout)
        .build();
    
    assert!(client.is_ok());
    // Timeout configuration is internal, we just verify build succeeds
}

#[test]
fn test_builder_custom_user_agent() {
    let client = ClientBuilder::new("https://metabase.example.com")
        .user_agent("MyApp/1.0")
        .build();
    
    assert!(client.is_ok());
    // User agent configuration is internal, we just verify build succeeds
}

#[test]
fn test_builder_invalid_url() {
    let client = ClientBuilder::new("not-a-valid-url")
        .build();
    
    assert!(client.is_err());
    if let Err(e) = client {
        assert!(e.to_string().contains("Invalid URL") || e.to_string().contains("invalid"));
    }
}

#[test]
fn test_builder_complete_configuration() {
    let client = ClientBuilder::new("https://metabase.example.com")
        .timeout(Duration::from_secs(45))
        .user_agent("TestClient/2.0")
        .build();
    
    assert!(client.is_ok());
    let client = client.unwrap();
    assert_eq!(client.base_url(), "https://metabase.example.com");
}