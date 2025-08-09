//! Service layer traits
//!
//! This module defines the core traits for the service layer.

use crate::core::error::Error;
use async_trait::async_trait;
use std::fmt::Debug;

/// Service-specific error type
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Business rule violation: {0}")]
    BusinessRule(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Repository error: {0}")]
    Repository(#[from] crate::repository::traits::RepositoryError),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<Error> for ServiceError {
    fn from(err: Error) -> Self {
        match err {
            Error::NotFound(msg) => ServiceError::NotFound(msg),
            Error::Authentication(msg) => ServiceError::Unauthorized(msg),
            Error::Validation(msg) => ServiceError::Validation(msg),
            _ => ServiceError::Other(err.to_string()),
        }
    }
}

impl From<ServiceError> for Error {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::NotFound(msg) => Error::NotFound(msg),
            ServiceError::Unauthorized(msg) => Error::Authentication(msg),
            ServiceError::Validation(msg) => Error::Validation(msg),
            _ => Error::Unknown(err.to_string()),
        }
    }
}

/// Service result type
pub type ServiceResult<T> = Result<T, ServiceError>;

/// Base service trait
#[async_trait]
pub trait Service: Send + Sync {
    /// Service name for identification
    fn name(&self) -> &str;

    /// Validate business rules
    async fn validate(&self) -> ServiceResult<()> {
        Ok(())
    }
}

/// Validation helpers
pub struct ValidationContext {
    errors: Vec<String>,
}

impl ValidationContext {
    /// Create a new validation context
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Add a validation error
    pub fn add_error(&mut self, error: impl Into<String>) {
        self.errors.push(error.into());
    }

    /// Check if validation passed
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get validation errors
    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    /// Convert to result
    pub fn to_result(self) -> ServiceResult<()> {
        if self.is_valid() {
            Ok(())
        } else {
            Err(ServiceError::Validation(self.errors.join(", ")))
        }
    }
}

impl Default for ValidationContext {
    fn default() -> Self {
        Self::new()
    }
}
