//! Core business logic layer
//!
//! This module contains the core domain models, error types, and business logic
//! that are independent of external dependencies.

pub mod auth;
pub mod error;
pub mod models;

// Re-export commonly used types
pub use error::{Error, Result};
