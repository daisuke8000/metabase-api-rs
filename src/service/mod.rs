//! Service layer for business logic and orchestration
//!
//! This module provides the service layer that encapsulates business logic
//! and orchestrates between repositories and other components.

pub mod card;
pub mod collection;
pub mod dashboard;
pub mod factory;
pub mod query;
pub mod traits;

pub use card::{CardService, HttpCardService};
pub use collection::{CollectionService, HttpCollectionService};
pub use dashboard::{DashboardService, HttpDashboardService};
pub use factory::{ServiceConfig, ServiceFactory};
pub use query::{HttpQueryService, QueryService};
pub use traits::{Service, ServiceError, ServiceResult};
