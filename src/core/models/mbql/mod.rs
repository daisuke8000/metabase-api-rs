//! MBQL (Metabase Query Language) query builder
//!
//! This module provides a type-safe, ergonomic API for building MBQL queries.

mod aggregation;
mod builder;
mod field_ref;
mod filter;
mod query;

pub use aggregation::{Aggregation, AggregationType};
pub use builder::MbqlQueryBuilder;
pub use field_ref::{FieldRef, FieldType};
pub use filter::{Filter, FilterOperator};
pub use query::{MbqlQuery, OrderBy, OrderDirection};

// Re-export for convenience
pub use query::MbqlQuery as Query;
