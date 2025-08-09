//! Dashboard service implementation
//!
//! This module provides business logic for Dashboard operations.

use super::traits::{Service, ServiceError, ServiceResult, ValidationContext};
use crate::core::models::{common::DashboardId, Dashboard};
use crate::repository::{
    dashboard::{DashboardFilterParams, DashboardRepository},
    traits::PaginationParams,
};
use async_trait::async_trait;
use std::sync::Arc;

/// Service trait for Dashboard operations
#[async_trait]
pub trait DashboardService: Service {
    /// Get a dashboard by ID
    async fn get_dashboard(&self, id: DashboardId) -> ServiceResult<Dashboard>;

    /// List dashboards with filters
    async fn list_dashboards(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<DashboardFilterParams>,
    ) -> ServiceResult<Vec<Dashboard>>;

    /// Create a new dashboard
    async fn create_dashboard(&self, dashboard: Dashboard) -> ServiceResult<Dashboard>;

    /// Update a dashboard
    async fn update_dashboard(
        &self,
        id: DashboardId,
        dashboard: Dashboard,
    ) -> ServiceResult<Dashboard>;

    /// Delete a dashboard
    async fn delete_dashboard(&self, id: DashboardId) -> ServiceResult<()>;

    /// Archive a dashboard
    async fn archive_dashboard(&self, id: DashboardId) -> ServiceResult<()>;

    /// Unarchive a dashboard
    async fn unarchive_dashboard(&self, id: DashboardId) -> ServiceResult<()>;

    /// Duplicate a dashboard
    async fn duplicate_dashboard(
        &self,
        id: DashboardId,
        new_name: &str,
    ) -> ServiceResult<Dashboard>;

    /// Add a card to a dashboard
    async fn add_card_to_dashboard(
        &self,
        dashboard_id: DashboardId,
        card_data: &serde_json::Value,
    ) -> ServiceResult<serde_json::Value>;

    /// Remove a card from a dashboard
    async fn remove_card_from_dashboard(
        &self,
        dashboard_id: DashboardId,
        card_id: i32,
    ) -> ServiceResult<()>;

    /// Validate dashboard data
    async fn validate_dashboard(&self, dashboard: &Dashboard) -> ServiceResult<()>;
}

/// HTTP implementation of DashboardService
pub struct HttpDashboardService {
    repository: Arc<dyn DashboardRepository>,
}

impl HttpDashboardService {
    /// Create a new HTTP dashboard service
    pub fn new(repository: Arc<dyn DashboardRepository>) -> Self {
        Self { repository }
    }

    /// Validate dashboard business rules
    fn validate_dashboard_rules(&self, dashboard: &Dashboard) -> ServiceResult<()> {
        let mut context = ValidationContext::new();

        // Name validation
        if dashboard.name.trim().is_empty() {
            context.add_error("Dashboard name cannot be empty");
        }

        if dashboard.name.len() > 255 {
            context.add_error("Dashboard name cannot exceed 255 characters");
        }

        // Description validation
        if let Some(desc) = &dashboard.description {
            if desc.len() > 5000 {
                context.add_error("Dashboard description cannot exceed 5000 characters");
            }
        }

        // Cache TTL validation
        if let Some(ttl) = dashboard.cache_ttl {
            if ttl < 0 {
                context.add_error("Cache TTL cannot be negative");
            }
            if ttl > 86400 {
                // 24 hours
                context.add_error("Cache TTL cannot exceed 24 hours");
            }
        }

        context.to_result()
    }
}

#[async_trait]
impl Service for HttpDashboardService {
    fn name(&self) -> &str {
        "DashboardService"
    }
}

#[async_trait]
impl DashboardService for HttpDashboardService {
    async fn get_dashboard(&self, id: DashboardId) -> ServiceResult<Dashboard> {
        self.repository.get(&id).await.map_err(ServiceError::from)
    }

    async fn list_dashboards(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<DashboardFilterParams>,
    ) -> ServiceResult<Vec<Dashboard>> {
        self.repository
            .list_with_filters(pagination, filters)
            .await
            .map_err(ServiceError::from)
    }

    async fn create_dashboard(&self, dashboard: Dashboard) -> ServiceResult<Dashboard> {
        // Validate business rules
        self.validate_dashboard_rules(&dashboard)?;

        // Create via repository
        self.repository
            .create(&dashboard)
            .await
            .map_err(ServiceError::from)
    }

    async fn update_dashboard(
        &self,
        id: DashboardId,
        mut dashboard: Dashboard,
    ) -> ServiceResult<Dashboard> {
        // Ensure ID matches
        dashboard.id = Some(id);

        // Validate business rules
        self.validate_dashboard_rules(&dashboard)?;

        // Check if dashboard exists
        self.repository.get(&id).await.map_err(ServiceError::from)?;

        // Update via repository
        self.repository
            .update(&id, &dashboard)
            .await
            .map_err(ServiceError::from)
    }

    async fn delete_dashboard(&self, id: DashboardId) -> ServiceResult<()> {
        // Check if dashboard exists
        self.repository.get(&id).await.map_err(ServiceError::from)?;

        // Delete via repository
        self.repository
            .delete(&id)
            .await
            .map_err(ServiceError::from)
    }

    async fn archive_dashboard(&self, id: DashboardId) -> ServiceResult<()> {
        self.repository
            .archive(&id)
            .await
            .map_err(ServiceError::from)
    }

    async fn unarchive_dashboard(&self, id: DashboardId) -> ServiceResult<()> {
        self.repository
            .unarchive(&id)
            .await
            .map_err(ServiceError::from)
    }

    async fn duplicate_dashboard(
        &self,
        id: DashboardId,
        new_name: &str,
    ) -> ServiceResult<Dashboard> {
        // Validate new name
        if new_name.trim().is_empty() {
            return Err(ServiceError::Validation(
                "New dashboard name cannot be empty".to_string(),
            ));
        }

        self.repository
            .duplicate(&id, new_name)
            .await
            .map_err(ServiceError::from)
    }

    async fn add_card_to_dashboard(
        &self,
        dashboard_id: DashboardId,
        card_data: &serde_json::Value,
    ) -> ServiceResult<serde_json::Value> {
        // Validate card data
        if card_data.is_null() {
            return Err(ServiceError::Validation(
                "Card data cannot be null".to_string(),
            ));
        }

        self.repository
            .add_card(&dashboard_id, card_data)
            .await
            .map_err(ServiceError::from)
    }

    async fn remove_card_from_dashboard(
        &self,
        dashboard_id: DashboardId,
        card_id: i32,
    ) -> ServiceResult<()> {
        self.repository
            .remove_card(&dashboard_id, card_id)
            .await
            .map_err(ServiceError::from)
    }

    async fn validate_dashboard(&self, dashboard: &Dashboard) -> ServiceResult<()> {
        self.validate_dashboard_rules(dashboard)
    }
}
