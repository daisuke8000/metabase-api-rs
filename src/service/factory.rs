//! Service factory for creating service instances
//!
//! This module provides a factory for creating service instances with proper dependencies.

use super::{
    card::{CardService, HttpCardService},
    collection::{CollectionService, HttpCollectionService},
    dashboard::{DashboardService, HttpDashboardService},
    query::{HttpQueryService, QueryService},
};
use crate::repository::factory::{RepositoryConfig, RepositoryFactory};
use std::sync::Arc;

/// Service configuration
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    /// Repository configuration
    pub repository_config: RepositoryConfig,
    /// Enable validation
    pub enable_validation: bool,
    /// Enable business rule checks
    pub enable_business_rules: bool,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            repository_config: RepositoryConfig::default(),
            enable_validation: true,
            enable_business_rules: true,
        }
    }
}

/// Factory for creating service instances
pub struct ServiceFactory {
    _config: ServiceConfig,
    repository_factory: RepositoryFactory,
}

impl ServiceFactory {
    /// Create a new service factory
    pub fn new(config: ServiceConfig) -> Self {
        let repository_factory = RepositoryFactory::new(config.repository_config.clone());
        Self {
            _config: config,
            repository_factory,
        }
    }

    /// Create a card service
    pub fn create_card_service(&self) -> Arc<dyn CardService> {
        let repository = self.repository_factory.create_card_repository();
        Arc::new(HttpCardService::new(repository))
    }

    /// Create a dashboard service
    pub fn create_dashboard_service(&self) -> Arc<dyn DashboardService> {
        let repository = self.repository_factory.create_dashboard_repository();
        Arc::new(HttpDashboardService::new(repository))
    }

    /// Create a collection service
    pub fn create_collection_service(&self) -> Arc<dyn CollectionService> {
        let repository = self.repository_factory.create_collection_repository();
        Arc::new(HttpCollectionService::new(repository))
    }

    /// Create a query service
    pub fn create_query_service(&self) -> Arc<dyn QueryService> {
        let repository = self.repository_factory.create_query_repository();
        Arc::new(HttpQueryService::new(repository))
    }
}
