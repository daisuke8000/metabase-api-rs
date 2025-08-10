//! Card repository trait and implementations
//!
//! This module provides the repository abstraction for Card entities.

use super::traits::{
    FilterParams, PaginationParams, Repository, RepositoryError, RepositoryResult,
};
use crate::core::models::common::CardId;
use crate::core::models::Card;
use crate::transport::http_provider_safe::{HttpProviderExt, HttpProviderSafe};
use async_trait::async_trait;
use std::sync::Arc;

/// Card-specific filter parameters
#[derive(Debug, Clone, Default)]
pub struct CardFilterParams {
    /// Filter by f parameter
    pub f: Option<String>,
    /// Filter by model type
    pub model_type: Option<String>,
    /// Filter by archived status
    pub archived: Option<bool>,
    /// Filter by collection ID
    pub collection_id: Option<i32>,
}

impl CardFilterParams {
    /// Create new card filter params
    pub fn new() -> Self {
        Self::default()
    }

    /// Set collection ID filter
    pub fn with_collection(mut self, collection_id: i32) -> Self {
        self.collection_id = Some(collection_id);
        self
    }

    // Set database ID filter (removed - not in current struct)
    // pub fn with_database(mut self, database_id: i32) -> Self {
    //     self.database_id = Some(database_id);
    //     self
    // }
}

/// Repository trait for Card entities
#[async_trait]
pub trait CardRepository: Repository<Entity = Card, Id = CardId> + Send + Sync {
    /// Cast to Any for type checking in tests
    fn as_any(&self) -> &dyn std::any::Any;
    /// List cards with card-specific filters
    async fn list_with_filters(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<CardFilterParams>,
    ) -> RepositoryResult<Vec<Card>>;

    /// Get cards in a specific collection
    async fn get_by_collection(&self, collection_id: i32) -> RepositoryResult<Vec<Card>>;

    /// Search cards by query
    async fn search(&self, query: &str) -> RepositoryResult<Vec<Card>>;

    /// Archive a card
    async fn archive(&self, id: &CardId) -> RepositoryResult<()>;

    /// Unarchive a card
    async fn unarchive(&self, id: &CardId) -> RepositoryResult<()>;

    /// Copy a card
    async fn copy(&self, id: &CardId, new_name: &str) -> RepositoryResult<Card>;

    /// Execute a card's query
    async fn execute_query(
        &self,
        id: &CardId,
        parameters: Option<serde_json::Value>,
    ) -> RepositoryResult<crate::core::models::QueryResult>;

    /// Export card query results
    async fn export_query(
        &self,
        id: &CardId,
        format: crate::core::models::common::ExportFormat,
        parameters: Option<serde_json::Value>,
    ) -> RepositoryResult<Vec<u8>>;

    /// Execute a pivot query for a card
    async fn execute_pivot_query(
        &self,
        id: &CardId,
        parameters: Option<serde_json::Value>,
    ) -> RepositoryResult<crate::core::models::QueryResult>;
}

/// HTTP implementation of CardRepository
pub struct HttpCardRepository {
    http_provider: Arc<dyn HttpProviderSafe>,
}

impl HttpCardRepository {
    /// Create a new HTTP card repository
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
impl Repository for HttpCardRepository {
    type Entity = Card;
    type Id = CardId;

    async fn get(&self, id: &CardId) -> RepositoryResult<Card> {
        let path = format!("/api/card/{}", id.0);
        self.http_provider.get(&path).await.map_err(|e| e.into())
    }

    async fn list(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<FilterParams>,
    ) -> RepositoryResult<Vec<Card>> {
        let query = self.build_query_params(pagination, filters);
        let path = format!("/api/card{}", query);
        self.http_provider.get(&path).await.map_err(|e| e.into())
    }

    async fn create(&self, entity: &Card) -> RepositoryResult<Card> {
        self.http_provider
            .post("/api/card", entity)
            .await
            .map_err(|e| e.into())
    }

    async fn update(&self, id: &CardId, entity: &Card) -> RepositoryResult<Card> {
        let path = format!("/api/card/{}", id.0);
        self.http_provider
            .put(&path, entity)
            .await
            .map_err(|e| e.into())
    }

    async fn delete(&self, id: &CardId) -> RepositoryResult<()> {
        let path = format!("/api/card/{}", id.0);
        self.http_provider.delete(&path).await.map_err(|e| e.into())
    }
}

#[async_trait]
impl CardRepository for HttpCardRepository {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    async fn list_with_filters(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<CardFilterParams>,
    ) -> RepositoryResult<Vec<Card>> {
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
            if let Some(f_param) = f.f {
                params.push(format!("f={}", f_param));
            }
            if let Some(model_type) = f.model_type {
                params.push(format!("model_type={}", model_type));
            }
            if let Some(archived) = f.archived {
                params.push(format!("archived={}", archived));
            }
            if let Some(collection_id) = f.collection_id {
                params.push(format!("collection_id={}", collection_id));
            }
        }

        let path = if params.is_empty() {
            "/api/card".to_string()
        } else {
            format!("/api/card?{}", params.join("&"))
        };

        self.http_provider.get(&path).await.map_err(|e| e.into())
    }

    async fn get_by_collection(&self, collection_id: i32) -> RepositoryResult<Vec<Card>> {
        let path = format!("/api/collection/{}/items", collection_id);
        // This returns collection items, we need to filter for cards
        let _items: serde_json::Value = self
            .http_provider
            .get(&path)
            .await
            .map_err(RepositoryError::from)?;

        // Extract cards from the response
        // This is a simplified version, actual implementation would parse properly
        Ok(Vec::new())
    }

    async fn search(&self, query: &str) -> RepositoryResult<Vec<Card>> {
        let filters = FilterParams::new().with_query(query);
        self.list(None, Some(filters)).await
    }

    async fn archive(&self, id: &CardId) -> RepositoryResult<()> {
        let path = format!("/api/card/{}", id.0);
        let body = serde_json::json!({ "archived": true });
        self.http_provider
            .put(&path, &body)
            .await
            .map(|_: serde_json::Value| ())
            .map_err(|e| e.into())
    }

    async fn unarchive(&self, id: &CardId) -> RepositoryResult<()> {
        let path = format!("/api/card/{}", id.0);
        let body = serde_json::json!({ "archived": false });
        self.http_provider
            .put(&path, &body)
            .await
            .map(|_: serde_json::Value| ())
            .map_err(|e| e.into())
    }

    async fn copy(&self, id: &CardId, new_name: &str) -> RepositoryResult<Card> {
        let path = format!("/api/card/{}/copy", id.0);
        let body = serde_json::json!({ "name": new_name });
        self.http_provider
            .post(&path, &body)
            .await
            .map_err(|e| e.into())
    }

    async fn execute_query(
        &self,
        id: &CardId,
        parameters: Option<serde_json::Value>,
    ) -> RepositoryResult<crate::core::models::QueryResult> {
        let path = format!("/api/card/{}/query", id.0);
        let request = if let Some(params) = parameters {
            serde_json::json!({ "parameters": params })
        } else {
            serde_json::json!({})
        };
        self.http_provider
            .post(&path, &request)
            .await
            .map_err(|e| e.into())
    }

    async fn export_query(
        &self,
        id: &CardId,
        format: crate::core::models::common::ExportFormat,
        parameters: Option<serde_json::Value>,
    ) -> RepositoryResult<Vec<u8>> {
        let path = format!("/api/card/{}/query/{}", id.0, format.as_str());
        let request = if let Some(params) = parameters {
            serde_json::json!({ "parameters": params })
        } else {
            serde_json::json!({})
        };
        // Use post_binary for export operations that return binary data (CSV, XLSX, etc.)
        self.http_provider
            .post_binary(&path, request)
            .await
            .map_err(RepositoryError::from)
    }

    async fn execute_pivot_query(
        &self,
        id: &CardId,
        parameters: Option<serde_json::Value>,
    ) -> RepositoryResult<crate::core::models::QueryResult> {
        let path = format!("/api/card/pivot/{}/query", id.0);
        let request = if let Some(params) = parameters {
            serde_json::json!({ "parameters": params })
        } else {
            serde_json::json!({})
        };
        self.http_provider
            .post(&path, &request)
            .await
            .map_err(|e| e.into())
    }
}

/// Mock implementation of CardRepository for testing
pub struct MockCardRepository {
    cards: Arc<tokio::sync::RwLock<Vec<Card>>>,
    should_fail: bool,
}

impl MockCardRepository {
    /// Create a new mock card repository
    pub fn new() -> Self {
        Self {
            cards: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            should_fail: false,
        }
    }

    /// Set whether operations should fail
    pub fn set_should_fail(&mut self, should_fail: bool) {
        self.should_fail = should_fail;
    }

    /// Add a card to the mock repository
    pub async fn add_card(&self, card: Card) {
        let mut cards = self.cards.write().await;
        cards.push(card);
    }
}

impl Default for MockCardRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Repository for MockCardRepository {
    type Entity = Card;
    type Id = CardId;

    async fn get(&self, id: &CardId) -> RepositoryResult<Card> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let cards = self.cards.read().await;
        cards
            .iter()
            .find(|c| c.id == Some(*id))
            .cloned()
            .ok_or_else(|| RepositoryError::NotFound(format!("Card {} not found", id.0)))
    }

    async fn list(
        &self,
        _pagination: Option<PaginationParams>,
        _filters: Option<FilterParams>,
    ) -> RepositoryResult<Vec<Card>> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let cards = self.cards.read().await;
        Ok(cards.clone())
    }

    async fn create(&self, entity: &Card) -> RepositoryResult<Card> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut cards = self.cards.write().await;
        let mut new_card = entity.clone();
        // Generate a mock ID if not present
        if new_card.id.is_none() {
            new_card.id = Some(CardId((cards.len() + 1) as i32));
        }
        cards.push(new_card.clone());
        Ok(new_card)
    }

    async fn update(&self, id: &CardId, entity: &Card) -> RepositoryResult<Card> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut cards = self.cards.write().await;
        if let Some(card) = cards.iter_mut().find(|c| c.id == Some(*id)) {
            *card = entity.clone();
            card.id = Some(*id); // Ensure ID is preserved
            Ok(card.clone())
        } else {
            Err(RepositoryError::NotFound(format!(
                "Card {} not found",
                id.0
            )))
        }
    }

    async fn delete(&self, id: &CardId) -> RepositoryResult<()> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut cards = self.cards.write().await;
        let initial_len = cards.len();
        cards.retain(|c| c.id != Some(*id));

        if cards.len() < initial_len {
            Ok(())
        } else {
            Err(RepositoryError::NotFound(format!(
                "Card {} not found",
                id.0
            )))
        }
    }
}

#[async_trait]
impl CardRepository for MockCardRepository {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    async fn list_with_filters(
        &self,
        pagination: Option<PaginationParams>,
        _filters: Option<CardFilterParams>,
    ) -> RepositoryResult<Vec<Card>> {
        // For mock, just return all cards
        self.list(pagination, None).await
    }

    async fn get_by_collection(&self, collection_id: i32) -> RepositoryResult<Vec<Card>> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let cards = self.cards.read().await;
        Ok(cards
            .iter()
            .filter(|c| c.collection_id == Some(collection_id))
            .cloned()
            .collect())
    }

    async fn search(&self, query: &str) -> RepositoryResult<Vec<Card>> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let cards = self.cards.read().await;
        Ok(cards
            .iter()
            .filter(|c| {
                c.name.to_lowercase().contains(&query.to_lowercase())
                    || c.description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&query.to_lowercase()))
                        .unwrap_or(false)
            })
            .cloned()
            .collect())
    }

    async fn archive(&self, id: &CardId) -> RepositoryResult<()> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut cards = self.cards.write().await;
        if let Some(card) = cards.iter_mut().find(|c| c.id == Some(*id)) {
            card.archived = true;
            Ok(())
        } else {
            Err(RepositoryError::NotFound(format!(
                "Card {} not found",
                id.0
            )))
        }
    }

    async fn unarchive(&self, id: &CardId) -> RepositoryResult<()> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut cards = self.cards.write().await;
        if let Some(card) = cards.iter_mut().find(|c| c.id == Some(*id)) {
            card.archived = false;
            Ok(())
        } else {
            Err(RepositoryError::NotFound(format!(
                "Card {} not found",
                id.0
            )))
        }
    }

    async fn copy(&self, id: &CardId, new_name: &str) -> RepositoryResult<Card> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        let mut cards = self.cards.write().await;
        if let Some(original) = cards.iter().find(|c| c.id == Some(*id)) {
            let mut new_card = original.clone();
            new_card.id = Some(CardId((cards.len() + 1) as i32));
            new_card.name = new_name.to_string();
            cards.push(new_card.clone());
            Ok(new_card)
        } else {
            Err(RepositoryError::NotFound(format!(
                "Card {} not found",
                id.0
            )))
        }
    }

    async fn execute_query(
        &self,
        id: &CardId,
        _parameters: Option<serde_json::Value>,
    ) -> RepositoryResult<crate::core::models::QueryResult> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        // Check if card exists
        let cards = self.cards.read().await;
        if !cards.iter().any(|c| c.id == Some(*id)) {
            return Err(RepositoryError::NotFound(format!(
                "Card {} not found",
                id.0
            )));
        }

        // Return mock query result
        use crate::core::models::common::MetabaseId;
        use crate::core::models::query::{Column, QueryData, QueryStatus};

        Ok(crate::core::models::QueryResult {
            data: QueryData {
                rows: vec![vec![serde_json::json!(1), serde_json::json!("test")]],
                cols: vec![
                    Column {
                        name: "id".to_string(),
                        display_name: "ID".to_string(),
                        base_type: "type/Integer".to_string(),
                        effective_type: None,
                        semantic_type: None,
                        field_ref: None,
                    },
                    Column {
                        name: "name".to_string(),
                        display_name: "Name".to_string(),
                        base_type: "type/Text".to_string(),
                        effective_type: None,
                        semantic_type: None,
                        field_ref: None,
                    },
                ],
                native_form: None,
                insights: vec![],
            },
            database_id: MetabaseId(1),
            started_at: chrono::Utc::now(),
            finished_at: Some(chrono::Utc::now()),
            status: QueryStatus::Completed,
            row_count: Some(1),
            running_time: Some(100),
            json_query: serde_json::json!({}),
        })
    }

    async fn export_query(
        &self,
        id: &CardId,
        _format: crate::core::models::common::ExportFormat,
        _parameters: Option<serde_json::Value>,
    ) -> RepositoryResult<Vec<u8>> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        // Check if card exists
        let cards = self.cards.read().await;
        if !cards.iter().any(|c| c.id == Some(*id)) {
            return Err(RepositoryError::NotFound(format!(
                "Card {} not found",
                id.0
            )));
        }

        // Return mock CSV export data
        Ok(b"id,name\n1,Test\n2,Data".to_vec())
    }

    async fn execute_pivot_query(
        &self,
        id: &CardId,
        _parameters: Option<serde_json::Value>,
    ) -> RepositoryResult<crate::core::models::QueryResult> {
        if self.should_fail {
            return Err(RepositoryError::Other("Mock failure".to_string()));
        }

        // Check if card exists
        let cards = self.cards.read().await;
        if !cards.iter().any(|c| c.id == Some(*id)) {
            return Err(RepositoryError::NotFound(format!(
                "Card {} not found",
                id.0
            )));
        }

        // Return mock pivot query result
        use crate::core::models::common::MetabaseId;
        use crate::core::models::query::{Column, QueryData, QueryStatus};

        Ok(crate::core::models::QueryResult {
            data: QueryData {
                rows: vec![vec![serde_json::json!("category1"), serde_json::json!(100)]],
                cols: vec![
                    Column {
                        name: "category".to_string(),
                        display_name: "Category".to_string(),
                        base_type: "type/Text".to_string(),
                        effective_type: None,
                        semantic_type: None,
                        field_ref: None,
                    },
                    Column {
                        name: "total".to_string(),
                        display_name: "Total".to_string(),
                        base_type: "type/Integer".to_string(),
                        effective_type: None,
                        semantic_type: None,
                        field_ref: None,
                    },
                ],
                native_form: None,
                insights: vec![],
            },
            database_id: MetabaseId(1),
            started_at: chrono::Utc::now(),
            finished_at: Some(chrono::Utc::now()),
            status: QueryStatus::Completed,
            row_count: Some(1),
            running_time: Some(150),
            json_query: serde_json::json!({}),
        })
    }
}
