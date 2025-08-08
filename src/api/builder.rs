//! Client builder implementation

use crate::api::client::MetabaseClient;
use crate::core::error::{Error, Result};
use crate::transport::HttpClientBuilder;
use std::time::Duration;

/// Builder for creating MetabaseClient instances
#[derive(Debug)]
pub struct ClientBuilder {
    base_url: String,
    timeout: Option<Duration>,
    user_agent: Option<String>,
}

impl ClientBuilder {
    /// Creates a new ClientBuilder with the specified base URL
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            timeout: None,
            user_agent: None,
        }
    }

    /// Sets the request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Sets the user agent string
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Builds the MetabaseClient
    pub fn build(self) -> Result<MetabaseClient> {
        // Validate URL
        if !self.base_url.starts_with("http://") && !self.base_url.starts_with("https://") {
            return Err(Error::Config(
                "Invalid URL: must start with http:// or https://".to_string(),
            ));
        }

        // Create HTTP client with custom configuration
        let mut http_builder = HttpClientBuilder::new(&self.base_url);

        if let Some(timeout) = self.timeout {
            http_builder = http_builder.timeout(timeout);
        }

        if let Some(user_agent) = self.user_agent {
            http_builder = http_builder.user_agent(user_agent);
        }

        let http_client = http_builder.build()?;
        let auth_manager = crate::api::auth::AuthManager::new();

        Ok(MetabaseClient {
            http_client,
            auth_manager,
            base_url: self.base_url,
        })
    }
}
