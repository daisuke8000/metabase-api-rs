//! Authentication abstraction traits
//!
//! This module provides trait-based abstractions for authentication mechanisms,
//! allowing for testable and flexible authentication implementations.

use crate::core::error::Result;
use crate::core::models::User;
use async_trait::async_trait;
use std::time::Duration;

/// Authentication credentials
#[derive(Debug, Clone)]
pub enum Credentials {
    /// Email and password authentication
    EmailPassword { email: String, password: String },
    /// API key authentication
    ApiKey(String),
    /// Session token authentication (for refresh)
    SessionToken(String),
}

/// Authentication response
#[derive(Debug, Clone)]
pub struct AuthResponse {
    /// Session token for authenticated requests
    pub session_token: String,
    /// Authenticated user information
    pub user: User,
    /// Token expiry duration (if known)
    pub expires_in: Option<Duration>,
}

/// Authentication provider trait
///
/// This trait abstracts authentication operations, allowing for
/// different implementations (e.g., production, mock for testing)
#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// Authenticate with the given credentials
    async fn authenticate(&self, credentials: &Credentials) -> Result<AuthResponse>;

    /// Refresh an existing session
    async fn refresh_session(&self, session_token: &str) -> Result<AuthResponse>;

    /// Validate a session token
    async fn validate_token(&self, session_token: &str) -> Result<bool>;

    /// Logout and invalidate a session
    async fn logout(&self, session_token: &str) -> Result<()>;

    /// Get user information for a session
    async fn get_user(&self, session_token: &str) -> Result<User>;
}

/// Mock authentication provider for testing
#[derive(Debug, Clone)]
pub struct MockAuthProvider {
    /// Whether authentication should succeed
    pub should_succeed: bool,
    /// Mock user to return
    pub mock_user: Option<User>,
    /// Mock token to return
    pub mock_token: String,
}

impl Default for MockAuthProvider {
    fn default() -> Self {
        Self {
            should_succeed: true,
            mock_user: Some(User {
                id: crate::core::models::common::UserId(1),
                email: "test@example.com".to_string(),
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
                is_active: true,
                is_superuser: false,
                is_qbnewb: false,
                date_joined: chrono::Utc::now(),
                last_login: None,
                locale: None,
                google_auth: false,
                ldap_auth: false,
                common_name: Some("Test User".to_string()),
                group_ids: Vec::new(),
                login_attributes: None,
                user_group_memberships: Vec::new(),
            }),
            mock_token: "mock_session_token_123".to_string(),
        }
    }
}

#[async_trait]
impl AuthProvider for MockAuthProvider {
    async fn authenticate(&self, _credentials: &Credentials) -> Result<AuthResponse> {
        if !self.should_succeed {
            return Err(crate::core::error::Error::Authentication(
                "Mock authentication failed".to_string(),
            ));
        }

        Ok(AuthResponse {
            session_token: self.mock_token.clone(),
            user: self.mock_user.clone().unwrap_or_else(|| User {
                id: crate::core::models::common::UserId(1),
                email: "mock@example.com".to_string(),
                first_name: "Mock".to_string(),
                last_name: "User".to_string(),
                is_active: true,
                is_superuser: false,
                is_qbnewb: false,
                date_joined: chrono::Utc::now(),
                last_login: None,
                locale: None,
                google_auth: false,
                ldap_auth: false,
                common_name: Some("Mock User".to_string()),
                group_ids: Vec::new(),
                login_attributes: None,
                user_group_memberships: Vec::new(),
            }),
            expires_in: Some(Duration::from_secs(3600)),
        })
    }

    async fn refresh_session(&self, _session_token: &str) -> Result<AuthResponse> {
        self.authenticate(&Credentials::SessionToken(self.mock_token.clone()))
            .await
    }

    async fn validate_token(&self, session_token: &str) -> Result<bool> {
        Ok(self.should_succeed && session_token == self.mock_token)
    }

    async fn logout(&self, _session_token: &str) -> Result<()> {
        if !self.should_succeed {
            return Err(crate::core::error::Error::Authentication(
                "Mock logout failed".to_string(),
            ));
        }
        Ok(())
    }

    async fn get_user(&self, _session_token: &str) -> Result<User> {
        if !self.should_succeed {
            return Err(crate::core::error::Error::Authentication(
                "Mock get_user failed".to_string(),
            ));
        }

        self.mock_user
            .clone()
            .ok_or_else(|| crate::core::error::Error::Authentication("No user found".to_string()))
    }
}
