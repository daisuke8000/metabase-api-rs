//! Database Service Layer
//!
//! This module provides the service layer for database operations,
//! encapsulating business logic and orchestration.

use crate::core::models::database::DatabaseMetadata;
use crate::core::models::field::Field;
use crate::core::models::{MetabaseId, SyncResult};
use crate::repository::database::{
    DatabaseFilterParams, DatabaseRepository, HttpDatabaseRepository,
};
use crate::repository::traits::PaginationParams;
use crate::service::traits::{Service, ServiceError, ServiceResult};
use crate::transport::http_provider_safe::HttpProviderSafe;
use async_trait::async_trait;
use std::sync::Arc;

/// Service trait for database operations
#[async_trait]
pub trait DatabaseService: Service + Send + Sync {
    /// Get database metadata
    async fn get_database_metadata(&self, id: MetabaseId) -> ServiceResult<DatabaseMetadata>;

    /// Sync database schema
    async fn sync_database_schema(&self, id: MetabaseId) -> ServiceResult<SyncResult>;

    /// Get database fields
    async fn get_database_fields(&self, id: MetabaseId) -> ServiceResult<Vec<Field>>;

    /// Get database schemas
    async fn get_database_schemas(&self, id: MetabaseId) -> ServiceResult<Vec<String>>;

    /// List databases
    async fn list_databases(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<DatabaseFilterParams>,
    ) -> ServiceResult<Vec<DatabaseMetadata>>;

    /// Validate database ID
    async fn validate_database_id(&self, id: MetabaseId) -> ServiceResult<()>;
}

/// HTTP implementation of DatabaseService
pub struct HttpDatabaseService {
    repository: Arc<dyn DatabaseRepository>,
}

impl HttpDatabaseService {
    /// Creates a new HttpDatabaseService
    pub fn new(repository: Arc<dyn DatabaseRepository>) -> Self {
        Self { repository }
    }

    /// Creates a new HttpDatabaseService from HTTP provider
    pub fn from_http_provider(http_provider: Arc<dyn HttpProviderSafe>) -> Self {
        let repository = Arc::new(HttpDatabaseRepository::new(http_provider));
        Self { repository }
    }
}

#[async_trait]
impl Service for HttpDatabaseService {
    fn name(&self) -> &str {
        "DatabaseService"
    }
}

#[async_trait]
impl DatabaseService for HttpDatabaseService {
    async fn get_database_metadata(&self, id: MetabaseId) -> ServiceResult<DatabaseMetadata> {
        // Validate database ID
        self.validate_database_id(id).await?;

        // Get metadata from repository
        self.repository
            .get(&id)
            .await
            .map_err(ServiceError::Repository)
    }

    async fn sync_database_schema(&self, id: MetabaseId) -> ServiceResult<SyncResult> {
        // Validate database ID
        self.validate_database_id(id).await?;

        // Sync schema through repository
        self.repository
            .sync_database_schema(&id)
            .await
            .map_err(ServiceError::Repository)
    }

    async fn get_database_fields(&self, id: MetabaseId) -> ServiceResult<Vec<Field>> {
        // Validate database ID
        self.validate_database_id(id).await?;

        // Get fields from repository
        self.repository
            .get_database_fields(&id)
            .await
            .map_err(ServiceError::Repository)
    }

    async fn get_database_schemas(&self, id: MetabaseId) -> ServiceResult<Vec<String>> {
        // Validate database ID
        self.validate_database_id(id).await?;

        // Get schemas from repository
        self.repository
            .get_database_schemas(&id)
            .await
            .map_err(ServiceError::Repository)
    }

    async fn list_databases(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<DatabaseFilterParams>,
    ) -> ServiceResult<Vec<DatabaseMetadata>> {
        // List databases from repository
        self.repository
            .list_with_filters(pagination, filters)
            .await
            .map_err(ServiceError::Repository)
    }

    async fn validate_database_id(&self, id: MetabaseId) -> ServiceResult<()> {
        if id.0 < 1 {
            return Err(ServiceError::Validation(
                "Invalid database ID: must be positive".to_string(),
            ));
        }
        Ok(())
    }
}
