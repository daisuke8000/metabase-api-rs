//! Collection repository trait and implementations
//!
//! This module provides the repository abstraction for Collection entities.

use super::traits::{
    FilterParams, PaginationParams, Repository, RepositoryError, RepositoryResult,
};
use crate::core::models::common::CollectionId;
use crate::core::models::Collection;
use crate::transport::http_provider_safe::{HttpProviderExt, HttpProviderSafe};
use async_trait::async_trait;
use std::sync::Arc;

/// Collection-specific filter parameters
#[derive(Debug, Clone, Default)]
pub struct CollectionFilterParams {
    /// Base filters
    pub base: FilterParams,
    /// Filter by parent collection ID
    pub parent_id: Option<i32>,
    /// Filter by namespace (e.g., "snippets", "cards")
    pub namespace: Option<String>,
    /// Filter by personal collection
    pub personal_only: Option<bool>,
}

impl CollectionFilterParams {
    /// Create new collection filter params
    pub fn new() -> Self {
        Self::default()
    }

    /// Set parent ID filter
    pub fn with_parent(mut self, parent_id: i32) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Set namespace filter
    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    /// Set personal collection filter
    pub fn with_personal_only(mut self, personal_only: bool) -> Self {
        self.personal_only = Some(personal_only);
        self
    }
}

/// Repository trait for Collection entities
#[async_trait]
pub trait CollectionRepository:
    Repository<Entity = Collection, Id = CollectionId> + Send + Sync
{
    /// List collections with collection-specific filters
    async fn list_with_filters(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<CollectionFilterParams>,
    ) -> RepositoryResult<Vec<Collection>>;

    /// Get child collections of a specific collection
    async fn get_children(&self, parent_id: CollectionId) -> RepositoryResult<Vec<Collection>>;

    /// Get root collections
    async fn get_root_collections(&self) -> RepositoryResult<Vec<Collection>>;

    /// Get collections by parent ID
    async fn get_by_parent(
        &self,
        parent_id: Option<CollectionId>,
    ) -> RepositoryResult<Vec<Collection>>;

    /// Get collection permissions
    async fn get_permissions(&self, id: &CollectionId) -> RepositoryResult<serde_json::Value>;

    /// Update collection permissions
    async fn update_permissions(
        &self,
        id: &CollectionId,
        permissions: &serde_json::Value,
    ) -> RepositoryResult<()>;

    /// Move collection to another parent
    async fn move_collection(
        &self,
        id: &CollectionId,
        new_parent_id: Option<CollectionId>,
    ) -> RepositoryResult<Collection>;

    /// Archive a collection
    async fn archive(&self, id: &CollectionId) -> RepositoryResult<()>;

    /// Unarchive a collection
    async fn unarchive(&self, id: &CollectionId) -> RepositoryResult<()>;
}

/// HTTP implementation of CollectionRepository
pub struct HttpCollectionRepository {
    http_provider: Arc<dyn HttpProviderSafe>,
}

impl HttpCollectionRepository {
    /// Create a new HTTP collection repository
    pub fn new(http_provider: Arc<dyn HttpProviderSafe>) -> Self {
        Self { http_provider }
    }

    /// Convert filter params to query string
    fn build_query_params(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<FilterParams>,
    ) -> String {
        let mut params = Vec::new();

        if let Some(p) = pagination {
            if let Some(page) = p.page {
                params.push(format!("page={}", page));
            }
            if let Some(limit) = p.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(offset) = p.offset {
                params.push(format!("offset={}", offset));
            }
        }

        if let Some(f) = filters {
            if let Some(query) = f.query {
                params.push(format!("q={}", query.replace(' ', "+")));
            }
            if let Some(archived) = f.archived {
                params.push(format!("archived={}", archived));
            }
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

#[async_trait]
impl Repository for HttpCollectionRepository {
    type Entity = Collection;
    type Id = CollectionId;

    async fn get(&self, id: &CollectionId) -> RepositoryResult<Collection> {
        let path = format!("/api/collection/{}", id.0);
        self.http_provider.get(&path).await.map_err(|e| e.into())
    }

    async fn list(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<FilterParams>,
    ) -> RepositoryResult<Vec<Collection>> {
        let query = self.build_query_params(pagination, filters);
        let path = format!("/api/collection{}", query);
        self.http_provider.get(&path).await.map_err(|e| e.into())
    }

    async fn create(&self, entity: &Collection) -> RepositoryResult<Collection> {
        self.http_provider
            .post("/api/collection", entity)
            .await
            .map_err(|e| e.into())
    }

    async fn update(&self, id: &CollectionId, entity: &Collection) -> RepositoryResult<Collection> {
        let path = format!("/api/collection/{}", id.0);
        self.http_provider
            .put(&path, entity)
            .await
            .map_err(|e| e.into())
    }

    async fn delete(&self, id: &CollectionId) -> RepositoryResult<()> {
        let path = format!("/api/collection/{}", id.0);
        self.http_provider.delete(&path).await.map_err(|e| e.into())
    }
}

#[async_trait]
impl CollectionRepository for HttpCollectionRepository {
    async fn list_with_filters(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<CollectionFilterParams>,
    ) -> RepositoryResult<Vec<Collection>> {
        // Convert CollectionFilterParams to FilterParams
        let base_filters = filters.map(|f| f.base);
        self.list(pagination, base_filters).await
    }

    async fn get_children(&self, parent_id: CollectionId) -> RepositoryResult<Vec<Collection>> {
        let path = format!("/api/collection/{}/children", parent_id.0);
        self.http_provider.get(&path).await.map_err(|e| e.into())
    }

    async fn get_root_collections(&self) -> RepositoryResult<Vec<Collection>> {
        self.http_provider
            .get("/api/collection/root")
            .await
            .map_err(|e| e.into())
    }

    async fn get_by_parent(
        &self,
        parent_id: Option<CollectionId>,
    ) -> RepositoryResult<Vec<Collection>> {
        let path = match parent_id {
            Some(id) => format!("/api/collection?parent_id={}", id.0),
            None => "/api/collection?parent_id=".to_string(),
        };
        self.http_provider.get(&path).await.map_err(|e| e.into())
    }

    async fn get_permissions(&self, id: &CollectionId) -> RepositoryResult<serde_json::Value> {
        let path = format!("/api/collection/{}/permissions", id.0);
        self.http_provider.get(&path).await.map_err(|e| e.into())
    }

    async fn update_permissions(
        &self,
        id: &CollectionId,
        permissions: &serde_json::Value,
    ) -> RepositoryResult<()> {
        let path = format!("/api/collection/{}/permissions", id.0);
        self.http_provider
            .put(&path, permissions)
            .await
            .map(|_: serde_json::Value| ())
            .map_err(|e| e.into())
    }

    async fn move_collection(
        &self,
        id: &CollectionId,
        new_parent_id: Option<CollectionId>,
    ) -> RepositoryResult<Collection> {
        let path = format!("/api/collection/{}", id.0);
        let body = serde_json::json!({
            "parent_id": new_parent_id.map(|id| id.0)
        });
        self.http_provider
            .put(&path, &body)
            .await
            .map_err(|e| e.into())
    }

    async fn archive(&self, id: &CollectionId) -> RepositoryResult<()> {
        let path = format!("/api/collection/{}", id.0);
        let body = serde_json::json!({ "archived": true });
        self.http_provider
            .put(&path, &body)
            .await
            .map(|_: serde_json::Value| ())
            .map_err(|e| e.into())
    }

    async fn unarchive(&self, id: &CollectionId) -> RepositoryResult<()> {
        let path = format!("/api/collection/{}", id.0);
        let body = serde_json::json!({ "archived": false });
        self.http_provider
            .put(&path, &body)
            .await
            .map(|_: serde_json::Value| ())
            .map_err(|e| e.into())
    }
}

/// Mock implementation of CollectionRepository for testing
pub struct MockCollectionRepository {
    collections: Arc<tokio::sync::RwLock<Vec<Collection>>>,
    should_fail: bool,
}

impl MockCollectionRepository {
    /// Create a new mock collection repository
    pub fn new() -> Self {
        Self {
            collections: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            should_fail: false,
        }
    }

    /// Set whether operations should fail
    pub fn set_should_fail(&mut self, should_fail: bool) {
        self.should_fail = should_fail;
    }

    /// Add a collection to the mock repository
    pub async fn add_collection(&self, collection: Collection) {
        let mut collections = self.collections.write().await;
        collections.push(collection);
    }
}

impl Default for MockCollectionRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Repository for MockCollectionRepository {
    type Entity = Collection;
    type Id = CollectionId;

    async fn get(&self, id: &CollectionId) -> RepositoryResult<Collection> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let collections = self.collections.read().await;
        collections
            .iter()
            .find(|c| c.id == Some(*id))
            .cloned()
            .ok_or_else(|| RepositoryError::NotFound(format!("Collection {} not found", id.0)))
    }

    async fn list(
        &self,
        _pagination: Option<PaginationParams>,
        _filters: Option<FilterParams>,
    ) -> RepositoryResult<Vec<Collection>> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let collections = self.collections.read().await;
        Ok(collections.clone())
    }

    async fn create(&self, entity: &Collection) -> RepositoryResult<Collection> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut collections = self.collections.write().await;
        let mut new_collection = entity.clone();
        // Generate a mock ID if not present
        if new_collection.id.is_none() {
            new_collection.id = Some(CollectionId((collections.len() + 1) as i32));
        }
        collections.push(new_collection.clone());
        Ok(new_collection)
    }

    async fn update(&self, id: &CollectionId, entity: &Collection) -> RepositoryResult<Collection> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut collections = self.collections.write().await;
        if let Some(collection) = collections.iter_mut().find(|c| c.id == Some(*id)) {
            *collection = entity.clone();
            collection.id = Some(*id); // Ensure ID is preserved
            Ok(collection.clone())
        } else {
            Err(RepositoryError::NotFound(format!(
                "Collection {} not found",
                id.0
            )))
        }
    }

    async fn delete(&self, id: &CollectionId) -> RepositoryResult<()> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut collections = self.collections.write().await;
        let initial_len = collections.len();
        collections.retain(|c| c.id != Some(*id));

        if collections.len() < initial_len {
            Ok(())
        } else {
            Err(RepositoryError::NotFound(format!(
                "Collection {} not found",
                id.0
            )))
        }
    }
}

#[async_trait]
impl CollectionRepository for MockCollectionRepository {
    async fn list_with_filters(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<CollectionFilterParams>,
    ) -> RepositoryResult<Vec<Collection>> {
        let base_filters = filters.map(|f| f.base);
        self.list(pagination, base_filters).await
    }

    async fn get_children(&self, parent_id: CollectionId) -> RepositoryResult<Vec<Collection>> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let collections = self.collections.read().await;
        Ok(collections
            .iter()
            .filter(|c| c.parent_id == Some(parent_id.0))
            .cloned()
            .collect())
    }

    async fn get_root_collections(&self) -> RepositoryResult<Vec<Collection>> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let collections = self.collections.read().await;
        Ok(collections
            .iter()
            .filter(|c| c.parent_id.is_none())
            .cloned()
            .collect())
    }

    async fn get_by_parent(
        &self,
        parent_id: Option<CollectionId>,
    ) -> RepositoryResult<Vec<Collection>> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let collections = self.collections.read().await;
        Ok(collections
            .iter()
            .filter(|c| match parent_id {
                Some(id) => c.parent_id == Some(id.0),
                None => c.parent_id.is_none(),
            })
            .cloned()
            .collect())
    }

    async fn get_permissions(&self, id: &CollectionId) -> RepositoryResult<serde_json::Value> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        // Verify collection exists
        self.get(id).await?;

        // Return mock permissions
        Ok(serde_json::json!({
            "read": ["all"],
            "write": ["admin"],
        }))
    }

    async fn update_permissions(
        &self,
        id: &CollectionId,
        _permissions: &serde_json::Value,
    ) -> RepositoryResult<()> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        // Verify collection exists
        self.get(id).await?;

        // In a real implementation, we would store the permissions
        Ok(())
    }

    async fn move_collection(
        &self,
        id: &CollectionId,
        new_parent_id: Option<CollectionId>,
    ) -> RepositoryResult<Collection> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut collections = self.collections.write().await;
        if let Some(collection) = collections.iter_mut().find(|c| c.id == Some(*id)) {
            collection.parent_id = new_parent_id.map(|id| id.0);
            Ok(collection.clone())
        } else {
            Err(RepositoryError::NotFound(format!(
                "Collection {} not found",
                id.0
            )))
        }
    }

    async fn archive(&self, id: &CollectionId) -> RepositoryResult<()> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut collections = self.collections.write().await;
        if let Some(collection) = collections.iter_mut().find(|c| c.id == Some(*id)) {
            collection.archived = Some(true);
            Ok(())
        } else {
            Err(RepositoryError::NotFound(format!(
                "Collection {} not found",
                id.0
            )))
        }
    }

    async fn unarchive(&self, id: &CollectionId) -> RepositoryResult<()> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut collections = self.collections.write().await;
        if let Some(collection) = collections.iter_mut().find(|c| c.id == Some(*id)) {
            collection.archived = Some(false);
            Ok(())
        } else {
            Err(RepositoryError::NotFound(format!(
                "Collection {} not found",
                id.0
            )))
        }
    }
}
