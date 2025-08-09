//! Core data models
//!
//! This module contains the data models used throughout the library

pub mod card;
pub mod collection;
pub mod common;
pub mod dashboard;
pub mod database;
pub mod field;
#[cfg(feature = "query-builder")]
pub mod mbql;
pub mod parameter;
pub mod query;
pub mod user;

// Re-export commonly used types
pub use card::{Card, CardBuilder, CardType};
pub use collection::{Collection, CollectionBuilder};
pub use common::{ExportFormat, MetabaseDateTime, MetabaseId, Pagination, UserId, Visibility};
pub use dashboard::{Dashboard, DashboardBuilder, DashboardCard, DashboardParameter};
pub use database::{
    ConnectionSource, Database, DatabaseBuilder, DatabaseMetadata, FieldMetadata, SyncResult,
    TableMetadata,
};
pub use field::Field;
pub use parameter::{Parameter, ParameterMapping, ParameterOption, ParameterTarget};
pub use query::{DatasetQuery, NativeQuery, QueryResult, QueryStatus};
pub use user::{HealthStatus, User, UserGroupMembership};
