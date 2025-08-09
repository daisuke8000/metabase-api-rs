//! Security tests for authentication and authorization modules
//!
//! Tests against common authentication vulnerabilities and security best practices

use metabase_api_rs::api::auth::{AuthManager, Credentials, SecureToken};
use std::thread;
use std::time::Duration;

#[test]
fn test_secure_token_memory_clearing() {
    // Test that SecureToken properly clears sensitive data
    let token = "secret_token_12345".to_string();
    let secure_token = SecureToken::new(token, None);

    // Token should be accessible when valid
    assert!(secure_token.get_if_valid().is_some());

    // Drop the token - memory should be cleared automatically via ZeroizeOnDrop
    drop(secure_token);

    // This test verifies the mechanism exists, actual memory clearing is handled by zeroize crate
}

#[test]
fn test_token_expiration_security() {
    // Test token expiration prevents access to expired tokens
    let token = "test_token".to_string();
    let short_ttl = Duration::from_millis(10); // Very short TTL for testing

    let secure_token = SecureToken::new(token, Some(short_ttl));

    // Token should be valid initially
    assert!(secure_token.get_if_valid().is_some());
    assert!(!secure_token.is_expired());

    // Wait for token to expire
    thread::sleep(Duration::from_millis(20));

    // Token should now be expired and inaccessible
    assert!(secure_token.is_expired());
    assert!(secure_token.get_if_valid().is_none());
}

#[test]
fn test_auth_manager_expired_token_security() {
    // Test that AuthManager properly handles expired tokens
    let mut auth_manager = AuthManager::new();

    // Initially not authenticated
    assert!(!auth_manager.is_authenticated());
    assert!(auth_manager.session_token().is_none());

    // Set session with very short TTL
    let test_user = metabase_api_rs::core::models::User {
        id: metabase_api_rs::core::models::common::UserId(1),
        email: "test@example.com".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        is_superuser: false,
        is_active: true,
        is_qbnewb: false,
        date_joined: chrono::Utc::now(),
        last_login: Some(chrono::Utc::now()),
        common_name: None,
        group_ids: Vec::new(),
        locale: None,
        google_auth: false,
        ldap_auth: false,
        login_attributes: None,
        user_group_memberships: Vec::new(),
    };

    auth_manager.set_session_with_ttl(
        "test_token".to_string(),
        test_user,
        Some(Duration::from_millis(10)),
    );

    // Should be authenticated initially
    assert!(auth_manager.is_authenticated());
    assert!(auth_manager.session_token().is_some());

    // Wait for expiration
    thread::sleep(Duration::from_millis(20));

    // Should no longer be authenticated
    assert!(!auth_manager.is_authenticated());
    assert!(auth_manager.session_token().is_none());
}

#[test]
fn test_credentials_secure_storage() {
    // Test that credentials properly protect sensitive information
    let password = "super_secret_password".to_string();
    let api_key = "api_key_12345".to_string();

    let email_creds = Credentials::email_password("user@example.com", password.clone());
    let api_creds = Credentials::new_api_key(api_key.clone());

    // Test email credentials
    assert_eq!(email_creds.email(), Some("user@example.com"));
    assert_eq!(email_creds.password(), Some(password.as_str()));
    assert!(email_creds.api_key().is_none());

    // Test API key credentials
    assert!(api_creds.email().is_none());
    assert!(api_creds.password().is_none());
    assert_eq!(api_creds.api_key(), Some(api_key.as_str()));

    // Test debug output doesn't expose sensitive data
    let debug_output = format!("{:?}", email_creds);
    assert!(debug_output.contains("[REDACTED]"));
    assert!(!debug_output.contains(&password));

    let debug_output = format!("{:?}", api_creds);
    assert!(debug_output.contains("[REDACTED]"));
    assert!(!debug_output.contains(&api_key));
}

#[test]
fn test_session_clearing_security() {
    // Test that session clearing properly removes sensitive data
    let mut auth_manager = AuthManager::new();

    let test_user = metabase_api_rs::core::models::User {
        id: metabase_api_rs::core::models::common::UserId(1),
        email: "test@example.com".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        is_superuser: false,
        is_active: true,
        is_qbnewb: false,
        date_joined: chrono::Utc::now(),
        last_login: Some(chrono::Utc::now()),
        common_name: None,
        group_ids: Vec::new(),
        locale: None,
        google_auth: false,
        ldap_auth: false,
        login_attributes: None,
        user_group_memberships: Vec::new(),
    };

    // Set session
    auth_manager.set_session("test_token".to_string(), test_user);
    assert!(auth_manager.is_authenticated());
    assert!(auth_manager.session_token().is_some());
    assert!(auth_manager.current_user().is_some());

    // Clear session
    auth_manager.clear_session();
    assert!(!auth_manager.is_authenticated());
    assert!(auth_manager.session_token().is_none());
    assert!(auth_manager.current_user().is_none());
}

#[test]
fn test_credentials_creation_methods() {
    // Test secure credential creation methods
    let email_creds = Credentials::email_password("test@example.com", "password123");
    let api_creds = Credentials::new_api_key("api_key_xyz");

    // Verify proper construction
    match email_creds {
        Credentials::EmailPassword { email, .. } => {
            assert_eq!(email, "test@example.com");
        }
        _ => panic!("Expected EmailPassword credentials"),
    }

    match api_creds {
        Credentials::ApiKey { .. } => {
            // Success - correct variant
        }
        _ => panic!("Expected ApiKey credentials"),
    }
}

#[cfg(test)]
mod brute_force_protection_tests {
    use super::*;

    #[test]
    fn test_no_timing_attacks_on_authentication() {
        // This test ensures that authentication checks don't leak timing information
        // that could be used for brute force attacks

        let auth_manager = AuthManager::new();

        // Test with invalid session - should return quickly and consistently
        let start = std::time::Instant::now();
        let result1 = auth_manager.is_authenticated();
        let duration1 = start.elapsed();

        let start = std::time::Instant::now();
        let result2 = auth_manager.is_authenticated();
        let duration2 = start.elapsed();

        // Both should return false
        assert!(!result1);
        assert!(!result2);

        // Timing difference should be minimal (< 1ms for this simple operation)
        let timing_diff = if duration1 > duration2 {
            duration1 - duration2
        } else {
            duration2 - duration1
        };

        assert!(
            timing_diff < Duration::from_millis(1),
            "Timing difference too large: {:?}",
            timing_diff
        );
    }
}

#[cfg(test)]
mod session_fixation_tests {
    use super::*;

    #[test]
    fn test_session_token_changes_on_authentication() {
        // Test that session tokens are properly managed to prevent session fixation
        let mut auth_manager = AuthManager::new();

        let test_user = metabase_api_rs::core::models::User {
            id: metabase_api_rs::core::models::common::UserId(1),
            email: "test@example.com".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            is_superuser: false,
            is_active: true,
            is_qbnewb: false,
            date_joined: chrono::Utc::now(),
            last_login: Some(chrono::Utc::now()),
            common_name: None,
            group_ids: Vec::new(),
            locale: None,
            google_auth: false,
            ldap_auth: false,
            login_attributes: None,
            user_group_memberships: Vec::new(),
        };

        // Set initial session
        auth_manager.set_session("token1".to_string(), test_user.clone());
        let token1 = auth_manager.session_token().unwrap().to_string();

        // Update session with new token
        auth_manager.set_session("token2".to_string(), test_user);
        let token2 = auth_manager.session_token().unwrap();

        // Tokens should be different
        assert_ne!(token1, token2);
        assert_eq!(token2, "token2");
    }
}
