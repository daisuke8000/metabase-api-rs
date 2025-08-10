//! Service Manager for dependency injection and service orchestration

use crate::repository::card::{CardRepository, HttpCardRepository};
use crate::repository::collection::{CollectionRepository, HttpCollectionRepository};
use crate::repository::dashboard::{DashboardRepository, HttpDashboardRepository};
use crate::repository::database::{DatabaseRepository, HttpDatabaseRepository};
use crate::repository::query::{HttpQueryRepository, QueryRepository};
use crate::service::auth::{AuthService, HttpAuthService};
use crate::service::card::{CardService, HttpCardService};
use crate::service::collection::{CollectionService, HttpCollectionService};
use crate::service::dashboard::{DashboardService, HttpDashboardService};
use crate::service::database::{DatabaseService, HttpDatabaseService};
use crate::service::query::{HttpQueryService, QueryService};
use crate::transport::http_provider_safe::HttpProviderSafe;
use std::sync::Arc;

/// Service Manager for dependency injection and service orchestration
#[derive(Clone)]
pub struct ServiceManager {
    auth_service: Arc<dyn AuthService>,
    card_service: Arc<dyn CardService>,
    collection_service: Arc<dyn CollectionService>,
    dashboard_service: Arc<dyn DashboardService>,
    database_service: Arc<dyn DatabaseService>,
    query_service: Arc<dyn QueryService>,
}

impl ServiceManager {
    /// Create a new ServiceManager with all services
    pub fn new(http_provider: Arc<dyn HttpProviderSafe>) -> Self {
        // Create auth service (no repository needed, uses http_provider directly)
        let auth_service = Arc::new(HttpAuthService::new(http_provider.clone()));

        // Create repositories
        let card_repo: Arc<dyn CardRepository> =
            Arc::new(HttpCardRepository::new(http_provider.clone()));
        let collection_repo: Arc<dyn CollectionRepository> =
            Arc::new(HttpCollectionRepository::new(http_provider.clone()));
        let dashboard_repo: Arc<dyn DashboardRepository> =
            Arc::new(HttpDashboardRepository::new(http_provider.clone()));
        let database_repo: Arc<dyn DatabaseRepository> =
            Arc::new(HttpDatabaseRepository::new(http_provider.clone()));
        let query_repo: Arc<dyn QueryRepository> =
            Arc::new(HttpQueryRepository::new(http_provider.clone()));

        // Create services with repositories
        let card_service = Arc::new(HttpCardService::new(card_repo));
        let collection_service = Arc::new(HttpCollectionService::new(collection_repo));
        let dashboard_service = Arc::new(HttpDashboardService::new(dashboard_repo));
        let database_service = Arc::new(HttpDatabaseService::new(database_repo));
        let query_service = Arc::new(HttpQueryService::new(query_repo));

        Self {
            auth_service,
            card_service,
            collection_service,
            dashboard_service,
            database_service,
            query_service,
        }
    }

    /// Get the auth service
    pub fn auth_service(&self) -> Option<Arc<dyn AuthService>> {
        Some(self.auth_service.clone())
    }

    /// Get the card service
    pub fn card_service(&self) -> Option<Arc<dyn CardService>> {
        Some(self.card_service.clone())
    }

    /// Get the collection service
    pub fn collection_service(&self) -> Option<Arc<dyn CollectionService>> {
        Some(self.collection_service.clone())
    }

    /// Get the dashboard service
    pub fn dashboard_service(&self) -> Option<Arc<dyn DashboardService>> {
        Some(self.dashboard_service.clone())
    }

    /// Get the database service
    pub fn database_service(&self) -> Option<Arc<dyn DatabaseService>> {
        Some(self.database_service.clone())
    }

    /// Get the query service
    pub fn query_service(&self) -> Option<Arc<dyn QueryService>> {
        Some(self.query_service.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::http_provider_safe::HttpProviderSafe;
    use std::sync::Arc;

    #[test]
    fn test_service_manager_creation() {
        // Arrange
        let mock_provider = create_mock_http_provider();

        // Act
        let manager = ServiceManager::new(mock_provider);

        // Assert
        assert!(manager.card_service().is_some());
        assert!(manager.collection_service().is_some());
        assert!(manager.dashboard_service().is_some());
    }

    #[test]
    fn test_service_dependency_injection() {
        // Arrange
        let mock_provider = create_mock_http_provider();
        let manager = ServiceManager::new(mock_provider);

        // Act
        let card_service = manager.card_service();

        // Assert
        // Verify that the service has the correct dependencies injected
        assert!(card_service.is_some());
    }

    #[test]
    fn test_service_lifecycle() {
        // Arrange
        let mock_provider = create_mock_http_provider();

        // Act
        let manager1 = ServiceManager::new(mock_provider.clone());
        let manager2 = ServiceManager::new(mock_provider);

        // Assert
        // Each manager should have its own service instances
        assert!(!Arc::ptr_eq(
            &manager1.card_service().unwrap(),
            &manager2.card_service().unwrap()
        ));
    }

    fn create_mock_http_provider() -> Arc<dyn HttpProviderSafe> {
        use crate::core::error::Result;
        use async_trait::async_trait;
        use serde_json::Value;

        struct MockHttpProvider;

        #[async_trait]
        impl HttpProviderSafe for MockHttpProvider {
            async fn get_json(&self, _path: &str) -> Result<Value> {
                Ok(Value::Null)
            }

            async fn post_json(&self, _path: &str, _body: Value) -> Result<Value> {
                Ok(Value::Null)
            }

            async fn put_json(&self, _path: &str, _body: Value) -> Result<Value> {
                Ok(Value::Null)
            }

            async fn delete_json(&self, _path: &str) -> Result<Value> {
                Ok(Value::Null)
            }

            async fn post_binary(&self, _path: &str, _body: Value) -> Result<Vec<u8>> {
                Ok(b"mock binary data".to_vec())
            }
        }

        Arc::new(MockHttpProvider)
    }
}
