//! Tests for AuthProvider trait and implementations

use metabase_api_rs::transport::{AuthProvider, Credentials, MockAuthProvider};

#[tokio::test]
async fn test_mock_auth_provider_success() {
    // Arrange
    let provider = MockAuthProvider::default();
    let credentials = Credentials::EmailPassword {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    // Act
    let result = provider.authenticate(&credentials).await;

    // Assert
    assert!(result.is_ok());
    let auth_response = result.unwrap();
    assert_eq!(auth_response.session_token, "mock_session_token_123");
    assert_eq!(auth_response.user.email, "test@example.com");
    assert!(auth_response.expires_in.is_some());
}

#[tokio::test]
async fn test_mock_auth_provider_failure() {
    // Arrange
    let mut provider = MockAuthProvider::default();
    provider.should_succeed = false;
    let credentials = Credentials::EmailPassword {
        email: "test@example.com".to_string(),
        password: "wrong_password".to_string(),
    };

    // Act
    let result = provider.authenticate(&credentials).await;

    // Assert
    assert!(result.is_err());
    match result.err() {
        Some(metabase_api_rs::core::error::Error::Authentication(msg)) => {
            assert_eq!(msg, "Mock authentication failed");
        }
        _ => panic!("Expected Authentication error"),
    }
}

#[tokio::test]
async fn test_mock_auth_provider_api_key() {
    // Arrange
    let provider = MockAuthProvider::default();
    let credentials = Credentials::ApiKey("test_api_key_456".to_string());

    // Act
    let result = provider.authenticate(&credentials).await;

    // Assert
    assert!(result.is_ok());
    let auth_response = result.unwrap();
    assert_eq!(auth_response.session_token, "mock_session_token_123");
}

#[tokio::test]
async fn test_mock_auth_provider_validate_token() {
    // Arrange
    let provider = MockAuthProvider::default();

    // Act
    let valid_result = provider.validate_token("mock_session_token_123").await;
    let invalid_result = provider.validate_token("invalid_token").await;

    // Assert
    assert!(valid_result.is_ok());
    assert!(valid_result.unwrap());

    assert!(invalid_result.is_ok());
    assert!(!invalid_result.unwrap());
}

#[tokio::test]
async fn test_mock_auth_provider_refresh_session() {
    // Arrange
    let provider = MockAuthProvider::default();

    // Act
    let result = provider.refresh_session("mock_session_token_123").await;

    // Assert
    assert!(result.is_ok());
    let auth_response = result.unwrap();
    assert_eq!(auth_response.session_token, "mock_session_token_123");
}

#[tokio::test]
async fn test_mock_auth_provider_logout() {
    // Arrange
    let provider = MockAuthProvider::default();

    // Act
    let result = provider.logout("mock_session_token_123").await;

    // Assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_mock_auth_provider_get_user() {
    // Arrange
    let provider = MockAuthProvider::default();

    // Act
    let result = provider.get_user("mock_session_token_123").await;

    // Assert
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.email, "test@example.com");
    assert_eq!(user.id, metabase_api_rs::core::models::common::UserId(1));
}

#[tokio::test]
async fn test_auth_provider_trait_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<MockAuthProvider>();

    // Also test that the trait object is Send + Sync
    fn assert_trait_object_send_sync() {
        let _: Box<dyn AuthProvider> = Box::new(MockAuthProvider::default());
    }
    assert_trait_object_send_sync();
}

#[tokio::test]
async fn test_credentials_variants() {
    // Test that all Credentials variants can be created
    let _email_pass = Credentials::EmailPassword {
        email: "user@example.com".to_string(),
        password: "pass123".to_string(),
    };

    let _api_key = Credentials::ApiKey("api_key_789".to_string());

    let _session = Credentials::SessionToken("session_abc".to_string());
}
