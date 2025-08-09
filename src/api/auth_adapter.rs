//! Adapter to integrate existing AuthManager with AuthProvider trait
//!
//! This module provides an adapter that allows the existing AuthManager
//! to work with the new AuthProvider trait abstraction.

use super::auth::{AuthManager, Credentials as ApiCredentials};
use crate::core::error::{Error, Result};
use crate::core::models::User;
use crate::transport::auth_traits::{
    AuthProvider, AuthResponse, Credentials as TransportCredentials,
};
use async_trait::async_trait;
use secrecy::ExposeSecret;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Adapter that implements AuthProvider for the existing AuthManager
pub struct AuthManagerAdapter {
    inner: Arc<RwLock<AuthManager>>,
    http_provider: Arc<dyn AuthProvider>,
}

impl AuthManagerAdapter {
    /// Create a new adapter with an AuthManager and HTTP provider
    pub fn new(auth_manager: AuthManager, http_provider: Arc<dyn AuthProvider>) -> Self {
        Self {
            inner: Arc::new(RwLock::new(auth_manager)),
            http_provider,
        }
    }

    /// Get a reference to the inner AuthManager (for read operations)
    pub async fn inner(&self) -> tokio::sync::RwLockReadGuard<'_, AuthManager> {
        self.inner.read().await
    }

    /// Get a mutable reference to the inner AuthManager (for write operations)
    pub async fn inner_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, AuthManager> {
        self.inner.write().await
    }

    /// Convert API credentials to transport credentials
    #[allow(dead_code)]
    fn convert_credentials(creds: &ApiCredentials) -> TransportCredentials {
        match creds {
            ApiCredentials::EmailPassword { email, password } => {
                TransportCredentials::EmailPassword {
                    email: email.clone(),
                    password: password.expose_secret().to_string(),
                }
            }
            ApiCredentials::ApiKey { key } => {
                TransportCredentials::ApiKey(key.expose_secret().to_string())
            }
        }
    }
}

#[async_trait]
impl AuthProvider for AuthManagerAdapter {
    async fn authenticate(&self, credentials: &TransportCredentials) -> Result<AuthResponse> {
        // Use the HTTP provider to perform actual authentication
        let response = self.http_provider.authenticate(credentials).await?;

        // Update the AuthManager with the session info
        let mut manager = self.inner.write().await;
        manager.set_session_with_ttl(
            response.session_token.clone(),
            response.user.clone(),
            response.expires_in,
        );

        Ok(response)
    }

    async fn refresh_session(&self, session_token: &str) -> Result<AuthResponse> {
        // Use the HTTP provider to refresh the session
        let response = self.http_provider.refresh_session(session_token).await?;

        // Update the AuthManager with the new session info
        let mut manager = self.inner.write().await;
        manager.set_session_with_ttl(
            response.session_token.clone(),
            response.user.clone(),
            response.expires_in,
        );

        Ok(response)
    }

    async fn validate_token(&self, session_token: &str) -> Result<bool> {
        // First check local state
        let manager = self.inner.read().await;
        if let Some(current_token) = manager.session_token() {
            if current_token != session_token {
                return Ok(false);
            }
            if !manager.is_authenticated() {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }
        drop(manager); // Release the read lock

        // Then validate with the server
        self.http_provider.validate_token(session_token).await
    }

    async fn logout(&self, session_token: &str) -> Result<()> {
        // Logout via HTTP provider
        self.http_provider.logout(session_token).await?;

        // Clear local session
        let mut manager = self.inner.write().await;
        manager.clear_session();

        Ok(())
    }

    async fn get_user(&self, session_token: &str) -> Result<User> {
        // First try to get from local cache
        let manager = self.inner.read().await;
        if let Some(user) = manager.current_user() {
            if let Some(current_token) = manager.session_token() {
                if current_token == session_token {
                    return Ok(user.clone());
                }
            }
        }
        drop(manager); // Release the read lock

        // Otherwise fetch from server
        let user = self.http_provider.get_user(session_token).await?;

        // Update local cache
        let mut manager = self.inner.write().await;
        if let Some(current_token) = manager.session_token() {
            if current_token == session_token {
                manager.set_session(session_token.to_string(), user.clone());
            }
        }

        Ok(user)
    }
}

/// Builder for AuthManagerAdapter
pub struct AuthManagerAdapterBuilder {
    auth_manager: Option<AuthManager>,
    http_provider: Option<Arc<dyn AuthProvider>>,
}

impl Default for AuthManagerAdapterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthManagerAdapterBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            auth_manager: None,
            http_provider: None,
        }
    }

    /// Set the AuthManager
    pub fn auth_manager(mut self, manager: AuthManager) -> Self {
        self.auth_manager = Some(manager);
        self
    }

    /// Set the HTTP provider
    pub fn http_provider(mut self, provider: Arc<dyn AuthProvider>) -> Self {
        self.http_provider = Some(provider);
        self
    }

    /// Build the adapter
    pub fn build(self) -> Result<AuthManagerAdapter> {
        let auth_manager = self.auth_manager.unwrap_or_default();
        let http_provider = self
            .http_provider
            .ok_or_else(|| Error::Config("HTTP provider is required".to_string()))?;

        Ok(AuthManagerAdapter::new(auth_manager, http_provider))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::auth_traits::MockAuthProvider;

    #[tokio::test]
    async fn test_adapter_authentication() {
        // Create a mock provider
        let mock_provider = Arc::new(MockAuthProvider::default());

        // Create an AuthManager
        let auth_manager = AuthManager::new();

        // Create the adapter
        let adapter = AuthManagerAdapter::new(auth_manager, mock_provider);

        // Test authentication
        let credentials = TransportCredentials::EmailPassword {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let response = adapter.authenticate(&credentials).await.unwrap();
        assert_eq!(response.session_token, "mock_session_token_123");

        // Verify the AuthManager was updated
        let manager = adapter.inner().await;
        assert!(manager.is_authenticated());
        assert_eq!(manager.session_token(), Some("mock_session_token_123"));
    }

    #[tokio::test]
    async fn test_adapter_logout() {
        // Create a mock provider
        let mock_provider = Arc::new(MockAuthProvider::default());

        // Create an AuthManager with a session
        let mut auth_manager = AuthManager::new();
        auth_manager.set_session(
            "test_token".to_string(),
            User {
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
            },
        );

        // Create the adapter
        let adapter = AuthManagerAdapter::new(auth_manager, mock_provider);

        // Verify initial state
        {
            let manager = adapter.inner().await;
            assert!(manager.is_authenticated());
        }

        // Logout
        adapter.logout("test_token").await.unwrap();

        // Verify session was cleared
        {
            let manager = adapter.inner().await;
            assert!(!manager.is_authenticated());
            assert!(manager.session_token().is_none());
        }
    }
}
