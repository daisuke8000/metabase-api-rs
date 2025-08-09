use metabase_api_rs::api::{AuthManager, ClientBuilder, Credentials, MetabaseClient};
use metabase_api_rs::core::models::{User, UserId};
use std::time::Duration;

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

// Builder tests
#[test]
fn test_builder_basic_configuration() {
    let client = ClientBuilder::new("https://metabase.example.com").build();

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
    let client = ClientBuilder::new("not-a-valid-url").build();

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

// Auth manager tests
#[test]
fn test_auth_manager_initial_state() {
    let auth_manager = AuthManager::new();

    assert!(!auth_manager.is_authenticated());
    assert!(auth_manager.session_token().is_none());
    assert!(auth_manager.current_user().is_none());
}

#[test]
fn test_session_management() {
    let mut auth_manager = AuthManager::new();

    // Set session
    let token = "test-session-token";
    let user = User {
        id: UserId(1),
        email: "user@example.com".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        is_superuser: false,
        is_active: true,
        is_qbnewb: false,
        date_joined: chrono::Utc::now(),
        last_login: Some(chrono::Utc::now()),
        common_name: Some("Test User".to_string()),
        group_ids: Vec::new(),
        locale: None,
        google_auth: false,
        ldap_auth: false,
        login_attributes: None,
        user_group_memberships: Vec::new(),
    };

    auth_manager.set_session(token.to_string(), user.clone());

    assert!(auth_manager.is_authenticated());
    assert_eq!(auth_manager.session_token(), Some(token));
    assert_eq!(
        auth_manager.current_user().map(|u| u.email.clone()),
        Some("user@example.com".to_string())
    );
}

#[test]
fn test_clear_session() {
    let mut auth_manager = AuthManager::new();

    // Set session
    let token = "test-session-token";
    let user = User {
        id: UserId(1),
        email: "user@example.com".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        is_superuser: false,
        is_active: true,
        is_qbnewb: false,
        date_joined: chrono::Utc::now(),
        last_login: Some(chrono::Utc::now()),
        common_name: Some("Test User".to_string()),
        group_ids: Vec::new(),
        locale: None,
        google_auth: false,
        ldap_auth: false,
        login_attributes: None,
        user_group_memberships: Vec::new(),
    };

    auth_manager.set_session(token.to_string(), user);
    assert!(auth_manager.is_authenticated());

    // Clear session
    auth_manager.clear_session();
    assert!(!auth_manager.is_authenticated());
    assert!(auth_manager.session_token().is_none());
    assert!(auth_manager.current_user().is_none());
}

#[test]
fn test_credentials_email_password() {
    let creds = Credentials::email_password("user@example.com", "password123");

    // Test accessor methods instead of direct field access
    assert_eq!(creds.email(), Some("user@example.com"));
    assert_eq!(creds.password(), Some("password123"));
    assert!(creds.api_key().is_none());
}

#[test]
fn test_credentials_api_key() {
    let creds = Credentials::new_api_key("mb_test_key_12345");

    // Test accessor methods instead of direct field access
    assert!(creds.email().is_none());
    assert!(creds.password().is_none());
    assert_eq!(creds.api_key(), Some("mb_test_key_12345"));
}
