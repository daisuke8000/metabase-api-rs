//! Repository layer - Data access abstraction
//!
//! This module provides trait-based abstractions for data access,
//! separating business logic from data retrieval/persistence.

pub mod card;
pub mod collection;
pub mod dashboard;
pub mod database;
pub mod factory;
pub mod query;
pub mod traits;

// Re-export main types
pub use card::{CardRepository, HttpCardRepository, MockCardRepository};
pub use collection::{CollectionRepository, HttpCollectionRepository, MockCollectionRepository};
pub use dashboard::{DashboardRepository, HttpDashboardRepository, MockDashboardRepository};
pub use database::{DatabaseRepository, HttpDatabaseRepository};
pub use factory::{RepositoryConfig, RepositoryFactory};
pub use query::{HttpQueryRepository, MockQueryRepository, QueryRepository};
pub use traits::{
    FilterParams, PaginationParams, Repository, RepositoryError, RepositoryResult, SortOrder,
};
