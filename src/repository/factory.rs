//! Repository factory for creating repository instances
//!
//! This module provides a factory pattern implementation for creating
//! repository instances with proper dependency injection.

use super::{
    card::{CardRepository, HttpCardRepository, MockCardRepository},
    collection::{CollectionRepository, HttpCollectionRepository, MockCollectionRepository},
    dashboard::{DashboardRepository, HttpDashboardRepository, MockDashboardRepository},
    query::{HttpQueryRepository, MockQueryRepository, QueryRepository},
};
use crate::transport::http_provider_safe::HttpProviderSafe;
use std::sync::Arc;

/// Repository configuration
#[derive(Clone, Default)]
pub struct RepositoryConfig {
    /// Use mock repositories for testing
    pub use_mocks: bool,
    /// HTTP provider for real repositories
    pub http_provider: Option<Arc<dyn HttpProviderSafe>>,
}

impl std::fmt::Debug for RepositoryConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RepositoryConfig")
            .field("use_mocks", &self.use_mocks)
            .field(
                "http_provider",
                &self.http_provider.as_ref().map(|_| "Arc<HttpProviderSafe>"),
            )
            .finish()
    }
}

impl RepositoryConfig {
    /// Create config for production use
    pub fn production(http_provider: Arc<dyn HttpProviderSafe>) -> Self {
        Self {
            use_mocks: false,
            http_provider: Some(http_provider),
        }
    }

    /// Create config for testing
    pub fn testing() -> Self {
        Self {
            use_mocks: true,
            http_provider: None,
        }
    }
}

/// Factory for creating repository instances
pub struct RepositoryFactory {
    config: RepositoryConfig,
}

impl RepositoryFactory {
    /// Create a new repository factory
    pub fn new(config: RepositoryConfig) -> Self {
        Self { config }
    }

    /// Create a card repository
    pub fn create_card_repository(&self) -> Arc<dyn CardRepository> {
        if self.config.use_mocks {
            Arc::new(MockCardRepository::new())
        } else {
            let http_provider = self
                .config
                .http_provider
                .as_ref()
                .expect("HTTP provider required for non-mock repositories")
                .clone();
            Arc::new(HttpCardRepository::new(http_provider))
        }
    }

    /// Create a collection repository
    pub fn create_collection_repository(&self) -> Arc<dyn CollectionRepository> {
        if self.config.use_mocks {
            Arc::new(MockCollectionRepository::new())
        } else {
            let http_provider = self
                .config
                .http_provider
                .as_ref()
                .expect("HTTP provider required for non-mock repositories")
                .clone();
            Arc::new(HttpCollectionRepository::new(http_provider))
        }
    }

    /// Create a dashboard repository
    pub fn create_dashboard_repository(&self) -> Arc<dyn DashboardRepository> {
        if self.config.use_mocks {
            Arc::new(MockDashboardRepository::new())
        } else {
            let http_provider = self
                .config
                .http_provider
                .as_ref()
                .expect("HTTP provider required for non-mock repositories")
                .clone();
            Arc::new(HttpDashboardRepository::new(http_provider))
        }
    }

    /// Create a query repository
    pub fn create_query_repository(&self) -> Arc<dyn QueryRepository> {
        if self.config.use_mocks {
            Arc::new(MockQueryRepository::new())
        } else {
            let http_provider = self
                .config
                .http_provider
                .as_ref()
                .expect("HTTP provider required for non-mock repositories")
                .clone();
            Arc::new(HttpQueryRepository::new(http_provider))
        }
    }

    /// Create all repositories at once
    pub fn create_all(&self) -> RepositorySet {
        RepositorySet {
            card: self.create_card_repository(),
            collection: self.create_collection_repository(),
            dashboard: self.create_dashboard_repository(),
            query: self.create_query_repository(),
        }
    }
}

/// A set of all repository instances
pub struct RepositorySet {
    /// Card repository
    pub card: Arc<dyn CardRepository>,
    /// Collection repository
    pub collection: Arc<dyn CollectionRepository>,
    /// Dashboard repository
    pub dashboard: Arc<dyn DashboardRepository>,
    /// Query repository
    pub query: Arc<dyn QueryRepository>,
}

/// Builder for RepositoryFactory
pub struct RepositoryFactoryBuilder {
    use_mocks: bool,
    http_provider: Option<Arc<dyn HttpProviderSafe>>,
}

impl Default for RepositoryFactoryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RepositoryFactoryBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            use_mocks: false,
            http_provider: None,
        }
    }

    /// Use mock repositories
    pub fn with_mocks(mut self) -> Self {
        self.use_mocks = true;
        self
    }

    /// Set HTTP provider
    pub fn with_http_provider(mut self, provider: Arc<dyn HttpProviderSafe>) -> Self {
        self.http_provider = Some(provider);
        self.use_mocks = false;
        self
    }

    /// Build the factory
    pub fn build(self) -> RepositoryFactory {
        let config = RepositoryConfig {
            use_mocks: self.use_mocks,
            http_provider: self.http_provider,
        };
        RepositoryFactory::new(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_with_mocks() {
        let factory = RepositoryFactoryBuilder::new().with_mocks().build();

        let repos = factory.create_all();

        // Verify all repositories are created
        // The actual functionality is tested through the trait implementations
        let _ = repos.card;
        let _ = repos.collection;
        let _ = repos.dashboard;
        let _ = repos.query;
    }

    #[test]
    fn test_config_presets() {
        let test_config = RepositoryConfig::testing();
        assert!(test_config.use_mocks);
        assert!(test_config.http_provider.is_none());

        // Production config would require an actual HTTP provider
        // which we can't test here without dependencies
    }
}
