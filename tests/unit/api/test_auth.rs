use metabase_api_rs::api::{AuthManager, Credentials};
use metabase_api_rs::core::models::User;

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
        id: 1,
        email: "user@example.com".to_string(),
        first_name: Some("Test".to_string()),
        last_name: Some("User".to_string()),
        is_superuser: false,
        is_active: true,
        date_joined: chrono::Utc::now(),
        last_login: Some(chrono::Utc::now()),
        common_name: Some("Test User".to_string()),
    };
    
    auth_manager.set_session(token.to_string(), user.clone());
    
    assert!(auth_manager.is_authenticated());
    assert_eq!(auth_manager.session_token(), Some(token));
    assert_eq!(auth_manager.current_user().map(|u| u.email.clone()), Some("user@example.com".to_string()));
}

#[test]
fn test_clear_session() {
    let mut auth_manager = AuthManager::new();
    
    // Set session
    let token = "test-session-token";
    let user = User {
        id: 1,
        email: "user@example.com".to_string(),
        first_name: Some("Test".to_string()),
        last_name: Some("User".to_string()),
        is_superuser: false,
        is_active: true,
        date_joined: chrono::Utc::now(),
        last_login: Some(chrono::Utc::now()),
        common_name: Some("Test User".to_string()),
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
    let creds = Credentials::EmailPassword {
        email: "user@example.com".to_string(),
        password: "password123".to_string(),
    };
    
    match creds {
        Credentials::EmailPassword { email, password } => {
            assert_eq!(email, "user@example.com");
            assert_eq!(password, "password123");
        }
        _ => panic!("Expected EmailPassword variant"),
    }
}

#[test]
fn test_credentials_api_key() {
    let creds = Credentials::ApiKey {
        key: "mb_test_key_12345".to_string(),
    };
    
    match creds {
        Credentials::ApiKey { key } => {
            assert_eq!(key, "mb_test_key_12345");
        }
        _ => panic!("Expected ApiKey variant"),
    }
}