//! Common repository traits and types
//!
//! This module defines the base traits and common types used across all repositories.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;

/// Repository-specific error type
#[derive(Debug, Error)]
pub enum RepositoryError {
    /// Entity not found
    #[error("Entity not found: {0}")]
    NotFound(String),

    /// Invalid parameters
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Serialization/Deserialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Other error
    #[error("Repository error: {0}")]
    Other(String),
}

/// Repository result type
pub type RepositoryResult<T> = Result<T, RepositoryError>;

/// Convert from core Error to RepositoryError
impl From<crate::core::error::Error> for RepositoryError {
    fn from(err: crate::core::error::Error) -> Self {
        use crate::core::error::Error;
        match err {
            Error::NotFound(msg) => RepositoryError::NotFound(msg),
            Error::Network(msg) => RepositoryError::Network(msg),
            Error::Authentication(msg) => RepositoryError::Authentication(msg),
            Error::Validation(msg) => RepositoryError::InvalidParams(msg),
            Error::Http { status: 404, .. } => {
                RepositoryError::NotFound("Resource not found".to_string())
            }
            Error::Http { message, .. } => RepositoryError::Network(message),
            other => RepositoryError::Other(other.to_string()),
        }
    }
}

/// Pagination parameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaginationParams {
    /// Page number (1-based)
    pub page: Option<u32>,
    /// Items per page
    pub limit: Option<u32>,
    /// Offset (alternative to page)
    pub offset: Option<u32>,
}

impl PaginationParams {
    /// Create new pagination params
    pub fn new() -> Self {
        Self::default()
    }

    /// Set page number
    pub fn with_page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set limit
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set offset
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
}

/// Sort order
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Asc
    }
}

/// Generic filter parameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilterParams {
    /// Search query
    pub query: Option<String>,
    /// Filter by active/archived status
    pub archived: Option<bool>,
    /// Filter by creation date (ISO 8601)
    pub created_after: Option<String>,
    /// Filter by creation date (ISO 8601)
    pub created_before: Option<String>,
    /// Filter by update date (ISO 8601)
    pub updated_after: Option<String>,
    /// Filter by update date (ISO 8601)
    pub updated_before: Option<String>,
    /// Additional custom filters
    pub custom: Option<serde_json::Value>,
}

impl FilterParams {
    /// Create new filter params
    pub fn new() -> Self {
        Self::default()
    }

    /// Set search query
    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// Set archived filter
    pub fn with_archived(mut self, archived: bool) -> Self {
        self.archived = Some(archived);
        self
    }
}

/// Base repository trait
///
/// This trait defines common operations that all repositories should support.
#[async_trait]
pub trait Repository: Send + Sync {
    /// The entity type this repository manages
    type Entity: Send + Sync + Debug;

    /// The ID type for entities
    type Id: Send + Sync + Debug + Clone;

    /// Get an entity by ID
    async fn get(&self, id: &Self::Id) -> RepositoryResult<Self::Entity>;

    /// List entities with optional pagination and filters
    async fn list(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<FilterParams>,
    ) -> RepositoryResult<Vec<Self::Entity>>;

    /// Create a new entity
    async fn create(&self, entity: &Self::Entity) -> RepositoryResult<Self::Entity>;

    /// Update an existing entity
    async fn update(&self, id: &Self::Id, entity: &Self::Entity) -> RepositoryResult<Self::Entity>;

    /// Delete an entity
    async fn delete(&self, id: &Self::Id) -> RepositoryResult<()>;

    /// Check if an entity exists
    async fn exists(&self, id: &Self::Id) -> RepositoryResult<bool> {
        match self.get(id).await {
            Ok(_) => Ok(true),
            Err(RepositoryError::NotFound(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Count entities matching filters
    async fn count(&self, filters: Option<FilterParams>) -> RepositoryResult<u64> {
        let entities = self.list(None, filters).await?;
        Ok(entities.len() as u64)
    }
}

/// Paginated response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The items in this page
    pub items: Vec<T>,
    /// Total number of items
    pub total: u64,
    /// Current page (1-based)
    pub page: u32,
    /// Items per page
    pub limit: u32,
    /// Total number of pages
    pub total_pages: u32,
}

impl<T> PaginatedResponse<T> {
    /// Create a new paginated response
    pub fn new(items: Vec<T>, total: u64, page: u32, limit: u32) -> Self {
        let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
        Self {
            items,
            total,
            page,
            limit,
            total_pages,
        }
    }

    /// Check if there's a next page
    pub fn has_next(&self) -> bool {
        self.page < self.total_pages
    }

    /// Check if there's a previous page
    pub fn has_prev(&self) -> bool {
        self.page > 1
    }
}
