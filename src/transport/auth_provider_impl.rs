//! AuthProvider implementation using HttpClient
//!
//! This module provides a production implementation of the AuthProvider trait
//! using the existing HttpClient for actual API communication.

use super::auth_traits::{AuthProvider, AuthResponse, Credentials};
use super::http::HttpClient;
use crate::core::error::Result;
use crate::core::models::User;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Login request body for email/password authentication
#[derive(Debug, Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

/// Login response from the API
#[derive(Debug, Deserialize)]
struct LoginResponse {
    id: String,
}

/// Session properties response
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct SessionProperties {
    #[serde(rename = "auth-provider")]
    auth_provider: Option<String>,
    #[serde(rename = "auth-session-type")]
    auth_session_type: Option<String>,
    #[serde(rename = "auth-session-id")]
    auth_session_id: Option<String>,
}

/// Production AuthProvider implementation using HttpClient
pub struct HttpAuthProvider {
    client: HttpClient,
}

impl HttpAuthProvider {
    /// Create a new HttpAuthProvider
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// Parse session token from response headers or body
    fn extract_session_token(&self, login_response: &LoginResponse) -> String {
        // The session ID is the token
        login_response.id.clone()
    }
}

#[async_trait]
impl AuthProvider for HttpAuthProvider {
    async fn authenticate(&self, credentials: &Credentials) -> Result<AuthResponse> {
        match credentials {
            Credentials::EmailPassword { email, password } => {
                // Login via email/password
                let login_request = LoginRequest {
                    username: email.clone(),
                    password: password.clone(),
                };

                let login_response: LoginResponse =
                    self.client.post("/api/session", &login_request).await?;

                let session_token = self.extract_session_token(&login_response);

                // Get current user information
                let user = self.get_user(&session_token).await?;

                Ok(AuthResponse {
                    session_token,
                    user,
                    expires_in: Some(Duration::from_secs(14 * 24 * 60 * 60)), // 14 days default
                })
            }

            Credentials::ApiKey(api_key) => {
                // For API key auth, we use it directly as a session token
                // and validate by getting user info
                let user = self.get_user(api_key).await?;

                Ok(AuthResponse {
                    session_token: api_key.clone(),
                    user,
                    expires_in: None, // API keys don't expire
                })
            }

            Credentials::SessionToken(token) => {
                // Validate existing session
                if self.validate_token(token).await? {
                    let user = self.get_user(token).await?;
                    Ok(AuthResponse {
                        session_token: token.clone(),
                        user,
                        expires_in: Some(Duration::from_secs(14 * 24 * 60 * 60)),
                    })
                } else {
                    Err(crate::core::error::Error::Authentication(
                        "Invalid session token".to_string(),
                    ))
                }
            }
        }
    }

    async fn refresh_session(&self, session_token: &str) -> Result<AuthResponse> {
        // Metabase doesn't have explicit refresh - validate and return same token
        if self.validate_token(session_token).await? {
            let user = self.get_user(session_token).await?;
            Ok(AuthResponse {
                session_token: session_token.to_string(),
                user,
                expires_in: Some(Duration::from_secs(14 * 24 * 60 * 60)),
            })
        } else {
            Err(crate::core::error::Error::Authentication(
                "Session expired or invalid".to_string(),
            ))
        }
    }

    async fn validate_token(&self, session_token: &str) -> Result<bool> {
        // Try to get session properties to validate the token
        match self
            .client
            .get::<SessionProperties>(&format!(
                "/api/session/properties?session_id={}",
                session_token
            ))
            .await
        {
            Ok(_) => Ok(true),
            Err(crate::core::error::Error::Authentication(_)) => Ok(false),
            Err(crate::core::error::Error::Http { status: 401, .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn logout(&self, session_token: &str) -> Result<()> {
        // DELETE /api/session to logout
        self.client
            .delete(&format!("/api/session/{}", session_token))
            .await
    }

    async fn get_user(&self, _session_token: &str) -> Result<User> {
        // Get current user with session token
        // We need to add the session token as a header
        // For now, we'll use the /api/user/current endpoint

        // This requires modifying the request to include the session header
        // Since HttpClient doesn't support custom headers yet, we'll need to extend it
        // For now, return a placeholder error

        Err(crate::core::error::Error::NotImplemented(
            "get_user requires HttpClient header support".to_string(),
        ))
    }
}

/// Builder for HttpAuthProvider
pub struct HttpAuthProviderBuilder {
    base_url: String,
    timeout: Option<Duration>,
}

impl HttpAuthProviderBuilder {
    /// Create a new builder
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            timeout: None,
        }
    }

    /// Set request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Build the HttpAuthProvider
    pub fn build(self) -> Result<HttpAuthProvider> {
        let mut builder = super::http::HttpClientBuilder::new(self.base_url);

        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }

        let client = builder.build()?;
        Ok(HttpAuthProvider::new(client))
    }
}
