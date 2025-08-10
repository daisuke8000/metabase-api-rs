//! Client builder implementation

use crate::api::client::MetabaseClient;
use crate::core::error::{Error, Result};
use crate::transport::HttpClientBuilder;
use std::time::Duration;

#[cfg(feature = "cache")]
use crate::cache::CacheConfig;

/// Builder for creating MetabaseClient instances
#[derive(Debug)]
pub struct ClientBuilder {
    base_url: String,
    timeout: Option<Duration>,
    user_agent: Option<String>,
    #[cfg(feature = "cache")]
    cache_config: Option<CacheConfig>,
    #[cfg(feature = "cache")]
    cache_enabled: Option<bool>,
}

impl ClientBuilder {
    /// Creates a new ClientBuilder with the specified base URL
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            timeout: None,
            user_agent: None,
            #[cfg(feature = "cache")]
            cache_config: None,
            #[cfg(feature = "cache")]
            cache_enabled: None,
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

    /// Sets the cache configuration
    #[cfg(feature = "cache")]
    pub fn cache_config(mut self, config: CacheConfig) -> Self {
        self.cache_config = Some(config);
        self
    }

    /// Enables or disables the cache
    #[cfg(feature = "cache")]
    pub fn cache_enabled(mut self, enabled: bool) -> Self {
        self.cache_enabled = Some(enabled);
        self
    }

    /// Disables the cache
    #[cfg(feature = "cache")]
    pub fn disable_cache(mut self) -> Self {
        self.cache_enabled = Some(false);
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

        #[cfg(feature = "cache")]
        let cache = {
            let mut config = self.cache_config.unwrap_or_default();
            if let Some(enabled) = self.cache_enabled {
                config.enabled = enabled;
            }
            crate::cache::CacheLayer::new(config)
        };

        // Create HttpProviderSafe adapter for ServiceManager
        use crate::service::ServiceManager;
        use crate::transport::http_provider_safe::{HttpClientAdapter, HttpProviderSafe};
        use std::sync::Arc;

        let http_provider: Arc<dyn HttpProviderSafe> =
            Arc::new(HttpClientAdapter::new(http_client));
        let service_manager = ServiceManager::new(http_provider);

        Ok(MetabaseClient {
            auth_manager,
            base_url: self.base_url,
            service_manager,
            #[cfg(feature = "cache")]
            cache,
        })
    }
}
