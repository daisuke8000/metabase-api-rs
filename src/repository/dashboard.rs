//! Dashboard repository trait and implementations
//!
//! This module provides the repository abstraction for Dashboard entities.

use super::traits::{
    FilterParams, PaginationParams, Repository, RepositoryError, RepositoryResult,
};
use crate::core::models::common::DashboardId;
use crate::core::models::Dashboard;
use crate::transport::http_provider_safe::{HttpProviderExt, HttpProviderSafe};
use async_trait::async_trait;
use std::sync::Arc;

/// Dashboard-specific filter parameters
#[derive(Debug, Clone, Default)]
pub struct DashboardFilterParams {
    /// Base filters
    pub base: FilterParams,
    /// Filter by collection ID
    pub collection_id: Option<i32>,
    /// Filter by creator ID
    pub creator_id: Option<i32>,
    /// Filter by favorite status
    pub is_favorite: Option<bool>,
}

impl DashboardFilterParams {
    /// Create new dashboard filter params
    pub fn new() -> Self {
        Self::default()
    }

    /// Set collection ID filter
    pub fn with_collection(mut self, collection_id: i32) -> Self {
        self.collection_id = Some(collection_id);
        self
    }

    /// Set creator ID filter
    pub fn with_creator(mut self, creator_id: i32) -> Self {
        self.creator_id = Some(creator_id);
        self
    }

    /// Set favorite filter
    pub fn with_favorite(mut self, is_favorite: bool) -> Self {
        self.is_favorite = Some(is_favorite);
        self
    }
}

/// Repository trait for Dashboard entities
#[async_trait]
pub trait DashboardRepository:
    Repository<Entity = Dashboard, Id = DashboardId> + Send + Sync
{
    /// List dashboards with dashboard-specific filters
    async fn list_with_filters(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<DashboardFilterParams>,
    ) -> RepositoryResult<Vec<Dashboard>>;

    /// Get dashboards in a specific collection
    async fn get_by_collection(&self, collection_id: i32) -> RepositoryResult<Vec<Dashboard>>;

    /// Get dashboard cards (visualizations on the dashboard)
    async fn get_cards(&self, id: &DashboardId) -> RepositoryResult<Vec<serde_json::Value>>;

    /// Add a card to a dashboard
    async fn add_card(
        &self,
        id: &DashboardId,
        card_data: &serde_json::Value,
    ) -> RepositoryResult<serde_json::Value>;

    /// Remove a card from a dashboard
    async fn remove_card(&self, id: &DashboardId, card_id: i32) -> RepositoryResult<()>;

    /// Update card position/size on dashboard
    async fn update_card(
        &self,
        id: &DashboardId,
        card_id: i32,
        updates: &serde_json::Value,
    ) -> RepositoryResult<serde_json::Value>;

    /// Duplicate a dashboard
    async fn duplicate(&self, id: &DashboardId, new_name: &str) -> RepositoryResult<Dashboard>;

    /// Archive a dashboard
    async fn archive(&self, id: &DashboardId) -> RepositoryResult<()>;

    /// Unarchive a dashboard
    async fn unarchive(&self, id: &DashboardId) -> RepositoryResult<()>;

    /// Favorite a dashboard
    async fn favorite(&self, id: &DashboardId) -> RepositoryResult<()>;

    /// Unfavorite a dashboard
    async fn unfavorite(&self, id: &DashboardId) -> RepositoryResult<()>;
}

/// HTTP implementation of DashboardRepository
pub struct HttpDashboardRepository {
    http_provider: Arc<dyn HttpProviderSafe>,
}

impl HttpDashboardRepository {
    /// Create a new HTTP dashboard repository
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
impl Repository for HttpDashboardRepository {
    type Entity = Dashboard;
    type Id = DashboardId;

    async fn get(&self, id: &DashboardId) -> RepositoryResult<Dashboard> {
        let path = format!("/api/dashboard/{}", id.0);
        self.http_provider.get(&path).await.map_err(|e| e.into())
    }

    async fn list(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<FilterParams>,
    ) -> RepositoryResult<Vec<Dashboard>> {
        let query = self.build_query_params(pagination, filters);
        let path = format!("/api/dashboard{}", query);
        self.http_provider.get(&path).await.map_err(|e| e.into())
    }

    async fn create(&self, entity: &Dashboard) -> RepositoryResult<Dashboard> {
        self.http_provider
            .post("/api/dashboard", entity)
            .await
            .map_err(|e| e.into())
    }

    async fn update(&self, id: &DashboardId, entity: &Dashboard) -> RepositoryResult<Dashboard> {
        let path = format!("/api/dashboard/{}", id.0);
        self.http_provider
            .put(&path, entity)
            .await
            .map_err(|e| e.into())
    }

    async fn delete(&self, id: &DashboardId) -> RepositoryResult<()> {
        let path = format!("/api/dashboard/{}", id.0);
        self.http_provider.delete(&path).await.map_err(|e| e.into())
    }
}

#[async_trait]
impl DashboardRepository for HttpDashboardRepository {
    async fn list_with_filters(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<DashboardFilterParams>,
    ) -> RepositoryResult<Vec<Dashboard>> {
        // Convert DashboardFilterParams to FilterParams
        let base_filters = filters.map(|f| f.base);
        self.list(pagination, base_filters).await
    }

    async fn get_by_collection(&self, collection_id: i32) -> RepositoryResult<Vec<Dashboard>> {
        let path = format!("/api/collection/{}/items", collection_id);
        // This returns collection items, we need to filter for dashboards
        let _items: serde_json::Value = self
            .http_provider
            .get(&path)
            .await
            .map_err(RepositoryError::from)?;

        // Extract dashboards from the response
        // This is a simplified version, actual implementation would parse properly
        Ok(Vec::new())
    }

    async fn get_cards(&self, id: &DashboardId) -> RepositoryResult<Vec<serde_json::Value>> {
        let path = format!("/api/dashboard/{}/cards", id.0);
        self.http_provider.get(&path).await.map_err(|e| e.into())
    }

    async fn add_card(
        &self,
        id: &DashboardId,
        card_data: &serde_json::Value,
    ) -> RepositoryResult<serde_json::Value> {
        let path = format!("/api/dashboard/{}/cards", id.0);
        self.http_provider
            .post(&path, card_data)
            .await
            .map_err(|e| e.into())
    }

    async fn remove_card(&self, id: &DashboardId, card_id: i32) -> RepositoryResult<()> {
        let path = format!("/api/dashboard/{}/cards/{}", id.0, card_id);
        self.http_provider.delete(&path).await.map_err(|e| e.into())
    }

    async fn update_card(
        &self,
        id: &DashboardId,
        card_id: i32,
        updates: &serde_json::Value,
    ) -> RepositoryResult<serde_json::Value> {
        let path = format!("/api/dashboard/{}/cards/{}", id.0, card_id);
        self.http_provider
            .put(&path, updates)
            .await
            .map_err(|e| e.into())
    }

    async fn duplicate(&self, id: &DashboardId, new_name: &str) -> RepositoryResult<Dashboard> {
        let path = format!("/api/dashboard/{}/copy", id.0);
        let body = serde_json::json!({ "name": new_name });
        self.http_provider
            .post(&path, &body)
            .await
            .map_err(|e| e.into())
    }

    async fn archive(&self, id: &DashboardId) -> RepositoryResult<()> {
        let path = format!("/api/dashboard/{}", id.0);
        let body = serde_json::json!({ "archived": true });
        self.http_provider
            .put(&path, &body)
            .await
            .map(|_: serde_json::Value| ())
            .map_err(|e| e.into())
    }

    async fn unarchive(&self, id: &DashboardId) -> RepositoryResult<()> {
        let path = format!("/api/dashboard/{}", id.0);
        let body = serde_json::json!({ "archived": false });
        self.http_provider
            .put(&path, &body)
            .await
            .map(|_: serde_json::Value| ())
            .map_err(|e| e.into())
    }

    async fn favorite(&self, id: &DashboardId) -> RepositoryResult<()> {
        let path = format!("/api/dashboard/{}/favorite", id.0);
        self.http_provider
            .post(&path, &serde_json::json!({}))
            .await
            .map(|_: serde_json::Value| ())
            .map_err(|e| e.into())
    }

    async fn unfavorite(&self, id: &DashboardId) -> RepositoryResult<()> {
        let path = format!("/api/dashboard/{}/favorite", id.0);
        self.http_provider.delete(&path).await.map_err(|e| e.into())
    }
}

/// Mock implementation of DashboardRepository for testing
pub struct MockDashboardRepository {
    dashboards: Arc<tokio::sync::RwLock<Vec<Dashboard>>>,
    dashboard_cards:
        Arc<tokio::sync::RwLock<std::collections::HashMap<DashboardId, Vec<serde_json::Value>>>>,
    favorites: Arc<tokio::sync::RwLock<std::collections::HashSet<DashboardId>>>,
    should_fail: bool,
}

impl MockDashboardRepository {
    /// Create a new mock dashboard repository
    pub fn new() -> Self {
        Self {
            dashboards: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            dashboard_cards: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            favorites: Arc::new(tokio::sync::RwLock::new(std::collections::HashSet::new())),
            should_fail: false,
        }
    }

    /// Set whether operations should fail
    pub fn set_should_fail(&mut self, should_fail: bool) {
        self.should_fail = should_fail;
    }

    /// Add a dashboard to the mock repository
    pub async fn add_dashboard(&self, dashboard: Dashboard) {
        let mut dashboards = self.dashboards.write().await;
        dashboards.push(dashboard);
    }
}

impl Default for MockDashboardRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Repository for MockDashboardRepository {
    type Entity = Dashboard;
    type Id = DashboardId;

    async fn get(&self, id: &DashboardId) -> RepositoryResult<Dashboard> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let dashboards = self.dashboards.read().await;
        dashboards
            .iter()
            .find(|d| d.id == Some(*id))
            .cloned()
            .ok_or_else(|| RepositoryError::NotFound(format!("Dashboard {} not found", id.0)))
    }

    async fn list(
        &self,
        _pagination: Option<PaginationParams>,
        _filters: Option<FilterParams>,
    ) -> RepositoryResult<Vec<Dashboard>> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let dashboards = self.dashboards.read().await;
        Ok(dashboards.clone())
    }

    async fn create(&self, entity: &Dashboard) -> RepositoryResult<Dashboard> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut dashboards = self.dashboards.write().await;
        let mut new_dashboard = entity.clone();
        // Generate a mock ID if not present
        if new_dashboard.id.is_none() {
            new_dashboard.id = Some(DashboardId((dashboards.len() + 1) as i32));
        }
        dashboards.push(new_dashboard.clone());
        Ok(new_dashboard)
    }

    async fn update(&self, id: &DashboardId, entity: &Dashboard) -> RepositoryResult<Dashboard> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut dashboards = self.dashboards.write().await;
        if let Some(dashboard) = dashboards.iter_mut().find(|d| d.id == Some(*id)) {
            *dashboard = entity.clone();
            dashboard.id = Some(*id); // Ensure ID is preserved
            Ok(dashboard.clone())
        } else {
            Err(RepositoryError::NotFound(format!(
                "Dashboard {} not found",
                id.0
            )))
        }
    }

    async fn delete(&self, id: &DashboardId) -> RepositoryResult<()> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut dashboards = self.dashboards.write().await;
        let initial_len = dashboards.len();
        dashboards.retain(|d| d.id != Some(*id));

        if dashboards.len() < initial_len {
            // Also clean up related data
            let mut cards = self.dashboard_cards.write().await;
            cards.remove(id);
            let mut favorites = self.favorites.write().await;
            favorites.remove(id);
            Ok(())
        } else {
            Err(RepositoryError::NotFound(format!(
                "Dashboard {} not found",
                id.0
            )))
        }
    }
}

#[async_trait]
impl DashboardRepository for MockDashboardRepository {
    async fn list_with_filters(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<DashboardFilterParams>,
    ) -> RepositoryResult<Vec<Dashboard>> {
        let base_filters = filters.map(|f| f.base);
        self.list(pagination, base_filters).await
    }

    async fn get_by_collection(&self, collection_id: i32) -> RepositoryResult<Vec<Dashboard>> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let dashboards = self.dashboards.read().await;
        Ok(dashboards
            .iter()
            .filter(|d| d.collection_id == Some(collection_id))
            .cloned()
            .collect())
    }

    async fn get_cards(&self, id: &DashboardId) -> RepositoryResult<Vec<serde_json::Value>> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        // Verify dashboard exists
        self.get(id).await?;

        let cards = self.dashboard_cards.read().await;
        Ok(cards.get(id).cloned().unwrap_or_default())
    }

    async fn add_card(
        &self,
        id: &DashboardId,
        card_data: &serde_json::Value,
    ) -> RepositoryResult<serde_json::Value> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        // Verify dashboard exists
        self.get(id).await?;

        let mut cards = self.dashboard_cards.write().await;
        let dashboard_cards = cards.entry(*id).or_insert_with(Vec::new);

        // Add an ID to the card data
        let mut new_card = card_data.clone();
        if let serde_json::Value::Object(ref mut map) = new_card {
            map.insert(
                "id".to_string(),
                serde_json::json!(dashboard_cards.len() + 1),
            );
        }

        dashboard_cards.push(new_card.clone());
        Ok(new_card)
    }

    async fn remove_card(&self, id: &DashboardId, card_id: i32) -> RepositoryResult<()> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        // Verify dashboard exists
        self.get(id).await?;

        let mut cards = self.dashboard_cards.write().await;
        if let Some(dashboard_cards) = cards.get_mut(id) {
            dashboard_cards.retain(|card| {
                card.get("id")
                    .and_then(|v| v.as_i64())
                    .map(|id| id != card_id as i64)
                    .unwrap_or(true)
            });
            Ok(())
        } else {
            Ok(())
        }
    }

    async fn update_card(
        &self,
        id: &DashboardId,
        card_id: i32,
        updates: &serde_json::Value,
    ) -> RepositoryResult<serde_json::Value> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        // Verify dashboard exists
        self.get(id).await?;

        let mut cards = self.dashboard_cards.write().await;
        if let Some(dashboard_cards) = cards.get_mut(id) {
            for card in dashboard_cards.iter_mut() {
                if card
                    .get("id")
                    .and_then(|v| v.as_i64())
                    .map(|id| id == card_id as i64)
                    .unwrap_or(false)
                {
                    // Merge updates into the card
                    if let serde_json::Value::Object(card_map) = card {
                        if let serde_json::Value::Object(updates_map) = updates {
                            for (key, value) in updates_map {
                                card_map.insert(key.clone(), value.clone());
                            }
                        }
                    }
                    return Ok(card.clone());
                }
            }
        }

        Err(RepositoryError::NotFound(format!(
            "Card {} not found on dashboard {}",
            card_id, id.0
        )))
    }

    async fn duplicate(&self, id: &DashboardId, new_name: &str) -> RepositoryResult<Dashboard> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut dashboards = self.dashboards.write().await;
        if let Some(original) = dashboards.iter().find(|d| d.id == Some(*id)) {
            let mut new_dashboard = original.clone();
            new_dashboard.id = Some(DashboardId((dashboards.len() + 1) as i32));
            new_dashboard.name = new_name.to_string();

            // Clone cards as well
            let cards = self.dashboard_cards.read().await;
            if let Some(original_cards) = cards.get(id) {
                let mut cards_mut = self.dashboard_cards.write().await;
                cards_mut.insert(new_dashboard.id.unwrap(), original_cards.clone());
            }

            dashboards.push(new_dashboard.clone());
            Ok(new_dashboard)
        } else {
            Err(RepositoryError::NotFound(format!(
                "Dashboard {} not found",
                id.0
            )))
        }
    }

    async fn archive(&self, id: &DashboardId) -> RepositoryResult<()> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut dashboards = self.dashboards.write().await;
        if let Some(dashboard) = dashboards.iter_mut().find(|d| d.id == Some(*id)) {
            dashboard.archived = Some(true);
            Ok(())
        } else {
            Err(RepositoryError::NotFound(format!(
                "Dashboard {} not found",
                id.0
            )))
        }
    }

    async fn unarchive(&self, id: &DashboardId) -> RepositoryResult<()> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut dashboards = self.dashboards.write().await;
        if let Some(dashboard) = dashboards.iter_mut().find(|d| d.id == Some(*id)) {
            dashboard.archived = Some(false);
            Ok(())
        } else {
            Err(RepositoryError::NotFound(format!(
                "Dashboard {} not found",
                id.0
            )))
        }
    }

    async fn favorite(&self, id: &DashboardId) -> RepositoryResult<()> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        // Verify dashboard exists
        self.get(id).await?;

        let mut favorites = self.favorites.write().await;
        favorites.insert(*id);
        Ok(())
    }

    async fn unfavorite(&self, id: &DashboardId) -> RepositoryResult<()> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        // Verify dashboard exists
        self.get(id).await?;

        let mut favorites = self.favorites.write().await;
        favorites.remove(id);
        Ok(())
    }
}
