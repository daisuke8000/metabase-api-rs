use metabase_api_rs::api::{MetabaseClient, Credentials};
use metabase_api_rs::core::Result;
use mockito::{mock, Matcher};
use serde_json::json;

#[test]
fn test_client_creation_with_valid_url() {
    let url = "https://metabase.example.com";
    let client = MetabaseClient::new(url);
    
    assert!(client.is_ok());
    let client = client.unwrap();
    assert_eq!(client.base_url(), url);
    assert!(!client.is_authenticated());
}

#[test] 
fn test_client_creation_with_invalid_url() {
    let url = "not-a-valid-url";
    let client = MetabaseClient::new(url);
    
    assert!(client.is_err());
    if let Err(e) = client {
        assert!(e.to_string().contains("Invalid URL") || e.to_string().contains("invalid"));
    }
}

#[tokio::test]
async fn test_authentication_with_email_password() {
    let _m = mock("POST", "/api/session")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "id": "test-session-token",
            "email": "user@example.com",
            "first_name": "Test",
            "last_name": "User"
        }).to_string())
        .create();
    
    let url = &mockito::server_url();
    let mut client = MetabaseClient::new(url).unwrap();
    
    let credentials = Credentials::EmailPassword {
        email: "user@example.com".to_string(),
        password: "password123".to_string(),
    };
    
    let result = client.authenticate(credentials).await;
    assert!(result.is_ok());
    assert!(client.is_authenticated());
}

#[tokio::test]
async fn test_authentication_failure() {
    let _m = mock("POST", "/api/session")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "message": "Invalid email or password"
        }).to_string())
        .create();
    
    let url = &mockito::server_url();
    let mut client = MetabaseClient::new(url).unwrap();
    
    let credentials = Credentials::EmailPassword {
        email: "user@example.com".to_string(),
        password: "wrong_password".to_string(),
    };
    
    let result = client.authenticate(credentials).await;
    assert!(result.is_err());
    assert!(!client.is_authenticated());
    
    if let Err(e) = result {
        assert!(e.to_string().contains("Authentication") || e.to_string().contains("401"));
    }
}

#[tokio::test]
async fn test_logout() {
    let _m1 = mock("POST", "/api/session")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "id": "test-session-token",
            "email": "user@example.com"
        }).to_string())
        .create();
    
    let _m2 = mock("DELETE", "/api/session")
        .with_status(204)
        .create();
    
    let url = &mockito::server_url();
    let mut client = MetabaseClient::new(url).unwrap();
    
    // First authenticate
    let credentials = Credentials::EmailPassword {
        email: "user@example.com".to_string(),
        password: "password123".to_string(),
    };
    client.authenticate(credentials).await.unwrap();
    assert!(client.is_authenticated());
    
    // Then logout
    let result = client.logout().await;
    assert!(result.is_ok());
    assert!(!client.is_authenticated());
}

#[tokio::test]
async fn test_health_check() {
    let _m = mock("GET", "/api/health")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "status": "healthy",
            "database": "connected",
            "cache": "active"
        }).to_string())
        .create();
    
    let url = &mockito::server_url();
    let client = MetabaseClient::new(url).unwrap();
    
    let result = client.health_check().await;
    assert!(result.is_ok());
    
    let health = result.unwrap();
    assert_eq!(health.status, "healthy");
}

#[tokio::test]
async fn test_get_current_user_authenticated() {
    let _m1 = mock("POST", "/api/session")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "id": "test-session-token",
            "email": "user@example.com",
            "first_name": "Test",
            "last_name": "User"
        }).to_string())
        .create();
    
    let _m2 = mock("GET", "/api/user/current")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "id": 1,
            "email": "user@example.com",
            "first_name": "Test",
            "last_name": "User",
            "is_superuser": false
        }).to_string())
        .create();
    
    let url = &mockito::server_url();
    let mut client = MetabaseClient::new(url).unwrap();
    
    // Authenticate first
    let credentials = Credentials::EmailPassword {
        email: "user@example.com".to_string(),
        password: "password123".to_string(),
    };
    client.authenticate(credentials).await.unwrap();
    
    // Get current user
    let result = client.get_current_user().await;
    assert!(result.is_ok());
    
    let user = result.unwrap();
    assert_eq!(user.email, "user@example.com");
}

#[tokio::test]
async fn test_get_current_user_unauthenticated() {
    let url = &mockito::server_url();
    let client = MetabaseClient::new(url).unwrap();
    
    let result = client.get_current_user().await;
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert!(e.to_string().contains("authenticated") || e.to_string().contains("Unauthorized"));
    }
}