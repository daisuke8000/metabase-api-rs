//! Metabase client implementation

use crate::api::auth::{AuthManager, Credentials};
use crate::core::error::{Error, Result};
use crate::core::models::{HealthStatus, User};
use crate::transport::HttpClient;
use serde::Deserialize;
use serde_json::json;

/// The main client for interacting with Metabase API
#[derive(Debug, Clone)]
pub struct MetabaseClient {
    pub(super) http_client: HttpClient,
    pub(super) auth_manager: AuthManager,
    pub(super) base_url: String,
}

impl MetabaseClient {
    /// Creates a new MetabaseClient instance
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        let base_url = base_url.into();

        // Validate URL
        if !base_url.starts_with("http://") && !base_url.starts_with("https://") {
            return Err(Error::Config(
                "Invalid URL: must start with http:// or https://".to_string(),
            ));
        }

        let http_client = HttpClient::new(&base_url)?;
        let auth_manager = AuthManager::new();

        Ok(Self {
            http_client,
            auth_manager,
            base_url,
        })
    }

    /// Gets the base URL of the client
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Checks if the client is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.auth_manager.is_authenticated()
    }

    /// Authenticates with the Metabase API
    pub async fn authenticate(&mut self, credentials: Credentials) -> Result<()> {
        let request_body = match credentials {
            Credentials::EmailPassword { email, password } => {
                json!({
                    "username": email,
                    "password": password
                })
            }
            Credentials::ApiKey { key } => {
                json!({
                    "api_key": key
                })
            }
        };

        #[derive(Deserialize)]
        struct SessionResponse {
            id: String,
            #[serde(flatten)]
            user_data: serde_json::Value,
        }

        let response: SessionResponse =
            self.http_client.post("/api/session", &request_body).await?;

        // Parse user information
        let user = User {
            id: 1, // Will be properly parsed from response
            email: response.user_data["email"]
                .as_str()
                .unwrap_or("unknown@example.com")
                .to_string(),
            first_name: response.user_data["first_name"]
                .as_str()
                .map(|s| s.to_string()),
            last_name: response.user_data["last_name"]
                .as_str()
                .map(|s| s.to_string()),
            is_superuser: response.user_data["is_superuser"]
                .as_bool()
                .unwrap_or(false),
            is_active: true,
            date_joined: chrono::Utc::now(),
            last_login: Some(chrono::Utc::now()),
            common_name: None,
        };

        self.auth_manager.set_session(response.id, user);
        Ok(())
    }

    /// Logs out from the Metabase API
    pub async fn logout(&mut self) -> Result<()> {
        if !self.is_authenticated() {
            return Ok(());
        }

        // Send logout request
        self.http_client.delete("/api/session").await?;

        // Clear local session
        self.auth_manager.clear_session();
        Ok(())
    }

    /// Performs a health check on the Metabase API
    pub async fn health_check(&self) -> Result<HealthStatus> {
        self.http_client.get("/api/health").await
    }

    /// Gets the current authenticated user
    pub async fn get_current_user(&self) -> Result<User> {
        if !self.is_authenticated() {
            return Err(Error::Authentication("Not authenticated".to_string()));
        }

        self.http_client.get("/api/user/current").await
    }
}
