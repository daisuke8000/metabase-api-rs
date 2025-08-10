//! Database repository for data access
//!
//! This module provides the repository layer for database operations,
//! handling all data access and HTTP communication for database-related functionality.

use crate::core::models::database::DatabaseMetadata;
use crate::core::models::field::Field;
use crate::core::models::{MetabaseId, SyncResult};
use crate::repository::traits::{
    FilterParams, PaginationParams, Repository, RepositoryError, RepositoryResult,
};
use crate::transport::http_provider_safe::{HttpProviderExt, HttpProviderSafe};
use async_trait::async_trait;
use serde_json::json;
use std::sync::Arc;

/// Database-specific filter parameters
#[derive(Debug, Clone, Default)]
pub struct DatabaseFilterParams {
    pub engine: Option<String>,
    pub native_permissions: Option<String>,
}

impl DatabaseFilterParams {
    /// Convert to query parameters
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = vec![];
        if let Some(engine) = &self.engine {
            params.push(("engine".to_string(), engine.clone()));
        }
        if let Some(permissions) = &self.native_permissions {
            params.push(("native_permissions".to_string(), permissions.clone()));
        }
        params
    }
}

/// Database repository trait for database-specific operations
#[async_trait]
pub trait DatabaseRepository:
    Repository<Entity = DatabaseMetadata, Id = MetabaseId> + Send + Sync
{
    /// Sync database schema
    async fn sync_database_schema(&self, id: &MetabaseId) -> RepositoryResult<SyncResult>;

    /// Get database fields
    async fn get_database_fields(&self, id: &MetabaseId) -> RepositoryResult<Vec<Field>>;

    /// Get database schemas
    async fn get_database_schemas(&self, id: &MetabaseId) -> RepositoryResult<Vec<String>>;

    /// List databases with specific filters
    async fn list_with_filters(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<DatabaseFilterParams>,
    ) -> RepositoryResult<Vec<DatabaseMetadata>>;
}

/// HTTP implementation of DatabaseRepository
pub struct HttpDatabaseRepository {
    http_provider: Arc<dyn HttpProviderSafe>,
}

impl HttpDatabaseRepository {
    /// Creates a new HttpDatabaseRepository
    pub fn new(http_provider: Arc<dyn HttpProviderSafe>) -> Self {
        Self { http_provider }
    }
}

#[async_trait]
impl Repository for HttpDatabaseRepository {
    type Entity = DatabaseMetadata;
    type Id = MetabaseId;

    async fn get(&self, id: &Self::Id) -> RepositoryResult<Self::Entity> {
        let path = format!("/api/database/{}/metadata", id.0);
        self.http_provider
            .get(&path)
            .await
            .map_err(|e| RepositoryError::Network(e.to_string()))
    }

    async fn list(
        &self,
        pagination: Option<PaginationParams>,
        _filters: Option<FilterParams>,
    ) -> RepositoryResult<Vec<Self::Entity>> {
        // For database listing, we ignore generic FilterParams and use list_with_filters
        let mut path = "/api/database".to_string();
        let mut query_params = vec![];

        if let Some(pagination) = pagination {
            query_params.extend(pagination.to_query_params());
        }

        if !query_params.is_empty() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            path = format!("{}?{}", path, query_string);
        }

        self.http_provider
            .get(&path)
            .await
            .map_err(|e| RepositoryError::Network(e.to_string()))
    }

    async fn create(&self, _entity: &Self::Entity) -> RepositoryResult<Self::Entity> {
        // Database creation is typically done through admin interface
        Err(RepositoryError::Other(
            "Database creation not supported through API".to_string(),
        ))
    }

    async fn update(
        &self,
        _id: &Self::Id,
        _entity: &Self::Entity,
    ) -> RepositoryResult<Self::Entity> {
        // Database updates are typically done through admin interface
        Err(RepositoryError::Other(
            "Database updates not supported through API".to_string(),
        ))
    }

    async fn delete(&self, _id: &Self::Id) -> RepositoryResult<()> {
        // Database deletion is typically done through admin interface
        Err(RepositoryError::Other(
            "Database deletion not supported through API".to_string(),
        ))
    }
}

#[async_trait]
impl DatabaseRepository for HttpDatabaseRepository {
    async fn sync_database_schema(&self, id: &MetabaseId) -> RepositoryResult<SyncResult> {
        let path = format!("/api/database/{}/sync_schema", id.0);
        self.http_provider
            .post(&path, &json!({}))
            .await
            .map_err(|e| RepositoryError::Network(e.to_string()))
    }

    async fn get_database_fields(&self, id: &MetabaseId) -> RepositoryResult<Vec<Field>> {
        let path = format!("/api/database/{}/fields", id.0);
        self.http_provider
            .get(&path)
            .await
            .map_err(|e| RepositoryError::Network(e.to_string()))
    }

    async fn get_database_schemas(&self, id: &MetabaseId) -> RepositoryResult<Vec<String>> {
        let path = format!("/api/database/{}/schemas", id.0);
        self.http_provider
            .get(&path)
            .await
            .map_err(|e| RepositoryError::Network(e.to_string()))
    }

    async fn list_with_filters(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<DatabaseFilterParams>,
    ) -> RepositoryResult<Vec<DatabaseMetadata>> {
        let mut path = "/api/database".to_string();
        let mut query_params = vec![];

        if let Some(pagination) = pagination {
            query_params.extend(pagination.to_query_params());
        }

        if let Some(filters) = filters {
            query_params.extend(filters.to_query_params());
        }

        if !query_params.is_empty() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            path = format!("{}?{}", path, query_string);
        }

        self.http_provider
            .get(&path)
            .await
            .map_err(|e| RepositoryError::Network(e.to_string()))
    }
}
