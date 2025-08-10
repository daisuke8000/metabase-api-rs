//! Authentication service implementation
//!
//! This module provides business logic for authentication operations.

use super::traits::{Service, ServiceError, ServiceResult};
use crate::api::auth::Credentials;
use crate::core::models::User;
use crate::transport::http_provider_safe::{HttpProviderExt, HttpProviderSafe};
use async_trait::async_trait;
use secrecy::ExposeSecret;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

/// Service trait for Authentication operations
#[async_trait]
pub trait AuthService: Service {
    /// Authenticate with the API
    async fn authenticate(&self, credentials: Credentials) -> ServiceResult<(String, User)>;

    /// Logout from the API
    async fn logout(&self, session_id: &str) -> ServiceResult<()>;

    /// Get current user information
    async fn get_current_user(&self, session_id: &str) -> ServiceResult<User>;

    /// Validate session
    async fn validate_session(&self, session_id: &str) -> ServiceResult<bool>;

    /// Health check
    async fn health_check(&self) -> ServiceResult<crate::core::models::HealthStatus>;
}

/// HTTP implementation of AuthService
pub struct HttpAuthService {
    http_provider: Arc<dyn HttpProviderSafe>,
}

impl HttpAuthService {
    /// Create a new HTTP auth service
    pub fn new(http_provider: Arc<dyn HttpProviderSafe>) -> Self {
        Self { http_provider }
    }
}

#[async_trait]
impl Service for HttpAuthService {
    fn name(&self) -> &str {
        "AuthService"
    }
}

#[async_trait]
impl AuthService for HttpAuthService {
    async fn authenticate(&self, credentials: Credentials) -> ServiceResult<(String, User)> {
        let request_body = match &credentials {
            Credentials::EmailPassword { email, password } => {
                json!({
                    "username": email,
                    "password": password.expose_secret()
                })
            }
            Credentials::ApiKey { key } => {
                json!({
                    "api_key": key.expose_secret()
                })
            }
        };

        #[derive(Deserialize)]
        struct SessionResponse {
            id: String,
            #[serde(flatten)]
            user_data: serde_json::Value,
        }

        let response: SessionResponse = self
            .http_provider
            .post("/api/session", &request_body)
            .await
            .map_err(ServiceError::from)?;

        // Parse user information
        use crate::core::models::common::UserId;

        let user = User {
            id: UserId(response.user_data["id"].as_i64().unwrap_or(1)),
            email: response.user_data["email"]
                .as_str()
                .unwrap_or("unknown@example.com")
                .to_string(),
            first_name: response.user_data["first_name"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string(),
            last_name: response.user_data["last_name"]
                .as_str()
                .unwrap_or("User")
                .to_string(),
            is_superuser: response.user_data["is_superuser"]
                .as_bool()
                .unwrap_or(false),
            is_active: true,
            is_qbnewb: response.user_data["is_qbnewb"].as_bool().unwrap_or(false),
            date_joined: chrono::Utc::now(),
            last_login: Some(chrono::Utc::now()),
            common_name: None,
            group_ids: Vec::new(),
            locale: response.user_data["locale"].as_str().map(|s| s.to_string()),
            google_auth: response.user_data["google_auth"].as_bool().unwrap_or(false),
            ldap_auth: response.user_data["ldap_auth"].as_bool().unwrap_or(false),
            login_attributes: None,
            user_group_memberships: Vec::new(),
        };

        Ok((response.id, user))
    }

    async fn logout(&self, _session_id: &str) -> ServiceResult<()> {
        self.http_provider
            .delete_json("/api/session")
            .await
            .map(|_: serde_json::Value| ())
            .map_err(ServiceError::from)
    }

    async fn get_current_user(&self, _session_id: &str) -> ServiceResult<User> {
        self.http_provider
            .get("/api/user/current")
            .await
            .map_err(ServiceError::from)
    }

    async fn validate_session(&self, session_id: &str) -> ServiceResult<bool> {
        // Try to get current user to validate session
        match self.get_current_user(session_id).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn health_check(&self) -> ServiceResult<crate::core::models::HealthStatus> {
        self.http_provider
            .get("/api/health")
            .await
            .map_err(ServiceError::from)
    }
}
