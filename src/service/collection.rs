//! Collection service implementation
//!
//! This module provides business logic for Collection operations.

use super::traits::{Service, ServiceError, ServiceResult, ValidationContext};
use crate::core::models::{common::CollectionId, Collection};
use crate::repository::{
    collection::{CollectionFilterParams, CollectionRepository},
    traits::PaginationParams,
};
use async_trait::async_trait;
use std::sync::Arc;

/// Service trait for Collection operations
#[async_trait]
pub trait CollectionService: Service {
    /// Get a collection by ID
    async fn get_collection(&self, id: CollectionId) -> ServiceResult<Collection>;

    /// List collections with filters
    async fn list_collections(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<CollectionFilterParams>,
    ) -> ServiceResult<Vec<Collection>>;

    /// Create a new collection
    async fn create_collection(&self, collection: Collection) -> ServiceResult<Collection>;

    /// Update a collection
    async fn update_collection(
        &self,
        id: CollectionId,
        collection: Collection,
    ) -> ServiceResult<Collection>;

    /// Delete a collection
    async fn delete_collection(&self, id: CollectionId) -> ServiceResult<()>;

    /// Archive a collection
    async fn archive_collection(&self, id: CollectionId) -> ServiceResult<()>;

    /// Unarchive a collection
    async fn unarchive_collection(&self, id: CollectionId) -> ServiceResult<()>;

    /// Move a collection to a new parent
    async fn move_collection(
        &self,
        id: CollectionId,
        new_parent_id: Option<CollectionId>,
    ) -> ServiceResult<Collection>;

    /// Get root collections
    async fn get_root_collections(&self) -> ServiceResult<Vec<Collection>>;

    /// Get collections by parent
    async fn get_collections_by_parent(
        &self,
        parent_id: CollectionId,
    ) -> ServiceResult<Vec<Collection>>;

    /// Validate collection data
    async fn validate_collection(&self, collection: &Collection) -> ServiceResult<()>;
}

/// HTTP implementation of CollectionService
pub struct HttpCollectionService {
    repository: Arc<dyn CollectionRepository>,
}

impl HttpCollectionService {
    /// Create a new HTTP collection service
    pub fn new(repository: Arc<dyn CollectionRepository>) -> Self {
        Self { repository }
    }

    /// Validate collection business rules
    fn validate_collection_rules(&self, collection: &Collection) -> ServiceResult<()> {
        let mut context = ValidationContext::new();

        // Name validation
        if collection.name.trim().is_empty() {
            context.add_error("Collection name cannot be empty");
        }

        if collection.name.len() > 255 {
            context.add_error("Collection name cannot exceed 255 characters");
        }

        // Description validation
        if let Some(desc) = &collection.description {
            if desc.len() > 5000 {
                context.add_error("Collection description cannot exceed 5000 characters");
            }
        }

        // Color validation
        if let Some(color) = &collection.color {
            // Validate hex color format
            if !color.starts_with('#') || color.len() != 7 {
                context.add_error("Collection color must be in hex format (#RRGGBB)");
            }
        }

        // Slug validation
        if let Some(slug) = &collection.slug {
            if slug.contains(' ') {
                context.add_error("Collection slug cannot contain spaces");
            }
            if slug.len() > 100 {
                context.add_error("Collection slug cannot exceed 100 characters");
            }
        }

        context.to_result()
    }

    /// Check for circular references in collection hierarchy
    async fn check_circular_reference(
        &self,
        id: CollectionId,
        parent_id: Option<CollectionId>,
    ) -> ServiceResult<()> {
        if let Some(parent) = parent_id {
            if parent == id {
                return Err(ServiceError::BusinessRule(
                    "Cannot set collection as its own parent".to_string(),
                ));
            }

            // TODO: Implement full circular reference check by traversing the hierarchy
        }
        Ok(())
    }
}

#[async_trait]
impl Service for HttpCollectionService {
    fn name(&self) -> &str {
        "CollectionService"
    }
}

#[async_trait]
impl CollectionService for HttpCollectionService {
    async fn get_collection(&self, id: CollectionId) -> ServiceResult<Collection> {
        self.repository.get(&id).await.map_err(ServiceError::from)
    }

    async fn list_collections(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<CollectionFilterParams>,
    ) -> ServiceResult<Vec<Collection>> {
        self.repository
            .list_with_filters(pagination, filters)
            .await
            .map_err(ServiceError::from)
    }

    async fn create_collection(&self, collection: Collection) -> ServiceResult<Collection> {
        // Validate business rules
        self.validate_collection_rules(&collection)?;

        // Check parent exists if specified
        if let Some(parent_id) = collection.parent_id {
            self.repository
                .get(&CollectionId(parent_id))
                .await
                .map_err(|_| {
                    ServiceError::NotFound(format!("Parent collection {} not found", parent_id))
                })?;
        }

        // Create via repository
        self.repository
            .create(&collection)
            .await
            .map_err(ServiceError::from)
    }

    async fn update_collection(
        &self,
        id: CollectionId,
        mut collection: Collection,
    ) -> ServiceResult<Collection> {
        // Ensure ID matches
        collection.id = Some(id);

        // Validate business rules
        self.validate_collection_rules(&collection)?;

        // Check if collection exists
        self.repository.get(&id).await.map_err(ServiceError::from)?;

        // Check for circular reference if parent is being changed
        if let Some(parent_id) = collection.parent_id {
            self.check_circular_reference(id, Some(CollectionId(parent_id)))
                .await?;
        }

        // Update via repository
        self.repository
            .update(&id, &collection)
            .await
            .map_err(ServiceError::from)
    }

    async fn delete_collection(&self, id: CollectionId) -> ServiceResult<()> {
        // Check if collection exists
        let collection = self.repository.get(&id).await.map_err(ServiceError::from)?;

        // Check if collection is personal
        if collection.is_personal() {
            return Err(ServiceError::BusinessRule(
                "Cannot delete personal collections".to_string(),
            ));
        }

        // TODO: Check for child collections and items

        // Delete via repository
        self.repository
            .delete(&id)
            .await
            .map_err(ServiceError::from)
    }

    async fn archive_collection(&self, id: CollectionId) -> ServiceResult<()> {
        self.repository
            .archive(&id)
            .await
            .map_err(ServiceError::from)
    }

    async fn unarchive_collection(&self, id: CollectionId) -> ServiceResult<()> {
        self.repository
            .unarchive(&id)
            .await
            .map_err(ServiceError::from)
    }

    async fn move_collection(
        &self,
        id: CollectionId,
        new_parent_id: Option<CollectionId>,
    ) -> ServiceResult<Collection> {
        // Check for circular reference
        self.check_circular_reference(id, new_parent_id).await?;

        // Check if new parent exists
        if let Some(parent) = new_parent_id {
            self.repository.get(&parent).await.map_err(|_| {
                ServiceError::NotFound(format!("Parent collection {} not found", parent.0))
            })?;
        }

        self.repository
            .move_collection(&id, new_parent_id)
            .await
            .map_err(ServiceError::from)
    }

    async fn get_root_collections(&self) -> ServiceResult<Vec<Collection>> {
        self.repository
            .get_root_collections()
            .await
            .map_err(ServiceError::from)
    }

    async fn get_collections_by_parent(
        &self,
        parent_id: CollectionId,
    ) -> ServiceResult<Vec<Collection>> {
        self.repository
            .get_by_parent(Some(parent_id))
            .await
            .map_err(ServiceError::from)
    }

    async fn validate_collection(&self, collection: &Collection) -> ServiceResult<()> {
        self.validate_collection_rules(collection)
    }
}
