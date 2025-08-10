//! Metabase client implementation

use crate::api::auth::{AuthManager, Credentials};
use crate::api::CardListParams;
use crate::core::error::{Error, Result};
use crate::core::models::common::{ExportFormat, CardId};
use crate::service::ServiceManager;
use crate::transport::http_provider_safe::{HttpProviderSafe, HttpClientAdapter};
#[cfg(feature = "query-builder")]
use crate::core::models::mbql::MbqlQuery;
use crate::core::models::{
    Card, Collection, Dashboard, DatabaseMetadata, DatasetQuery, Field, HealthStatus, MetabaseId,
    NativeQuery, Pagination, QueryResult, SyncResult, User,
};
use crate::transport::HttpClient;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(feature = "cache")]
use crate::cache::{cache_key, CacheConfig, CacheLayer};

/// The main client for interacting with Metabase API
#[derive(Clone)]
pub struct MetabaseClient {
    pub(super) http_client: HttpClient,
    pub(super) auth_manager: AuthManager,
    pub(super) base_url: String,
    pub(super) service_manager: ServiceManager,
    #[cfg(feature = "cache")]
    pub(super) cache: CacheLayer,
}

impl MetabaseClient {
    /// Creates a new MetabaseClient instance
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        let base_url = base_url.into();

        // Validate URL
        if !base_url.starts_with("http://") && !base_url.starts_with("https://") {
            return Err(Error::Config(
                "Invalid URL: must start with http:// or https://".to_string(),
            ));
        }

        let http_client = HttpClient::new(&base_url)?;
        let auth_manager = AuthManager::new();
        
        // Create HttpProviderSafe adapter for ServiceManager
        let http_provider: Arc<dyn HttpProviderSafe> = Arc::new(HttpClientAdapter::new(http_client.clone()));
        let service_manager = ServiceManager::new(http_provider);

        Ok(Self {
            http_client,
            auth_manager,
            base_url,
            service_manager,
            #[cfg(feature = "cache")]
            cache: CacheLayer::new(CacheConfig::default()),
        })
    }

    /// Creates a new MetabaseClient with custom cache configuration
    #[cfg(feature = "cache")]
    pub fn with_cache(base_url: impl Into<String>, cache_config: CacheConfig) -> Result<Self> {
        let base_url = base_url.into();

        // Validate URL
        if !base_url.starts_with("http://") && !base_url.starts_with("https://") {
            return Err(Error::Config(
                "Invalid URL: must start with http:// or https://".to_string(),
            ));
        }

        let http_client = HttpClient::new(&base_url)?;
        let auth_manager = AuthManager::new();
        
        // Create HttpProviderSafe adapter for ServiceManager
        let http_provider: Arc<dyn HttpProviderSafe> = Arc::new(HttpClientAdapter::new(http_client.clone()));
        let service_manager = ServiceManager::new(http_provider);

        Ok(Self {
            http_client,
            auth_manager,
            base_url,
            service_manager,
            cache: CacheLayer::new(cache_config),
        })
    }

    /// Gets the base URL of the client
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Checks if the client is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.auth_manager.is_authenticated()
    }

    /// Checks if cache is enabled
    #[cfg(feature = "cache")]
    pub fn is_cache_enabled(&self) -> bool {
        self.cache.is_enabled()
    }

    /// Sets the cache enabled state
    #[cfg(feature = "cache")]
    pub fn set_cache_enabled(&mut self, enabled: bool) {
        self.cache.set_enabled(enabled);
    }

    /// Checks if cache is enabled (always false when cache feature is disabled)
    #[cfg(not(feature = "cache"))]
    pub fn is_cache_enabled(&self) -> bool {
        false
    }

    /// Sets the cache enabled state (no-op when cache feature is disabled)
    #[cfg(not(feature = "cache"))]
    pub fn set_cache_enabled(&mut self, _enabled: bool) {
        // No-op when cache feature is disabled
    }

    /// Authenticates with the Metabase API
    pub async fn authenticate(&mut self, credentials: Credentials) -> Result<()> {
        // Use ServiceManager for authentication
        let (session_id, user) = self.service_manager
            .auth_service()
            .ok_or_else(|| Error::Config("Auth service not available".to_string()))?
            .authenticate(credentials)
            .await
            .map_err(|e| Error::Config(format!("Service error: {}", e)))?;

        self.auth_manager.set_session(session_id, user);
        Ok(())
    }

    /// Logs out from the Metabase API
    pub async fn logout(&mut self) -> Result<()> {
        if !self.is_authenticated() {
            return Ok(());
        }

        // Get session ID before clearing
        let session_id = self.auth_manager.get_session_id();
        
        if let Some(id) = session_id {
            // Use ServiceManager for logout
            self.service_manager
                .auth_service()
                .ok_or_else(|| Error::Config("Auth service not available".to_string()))?
                .logout(&id)
                .await
                .map_err(|e| Error::Config(format!("Service error: {}", e)))?;
        }

        // Clear local session
        self.auth_manager.clear_session();
        Ok(())
    }

    /// Performs a health check on the Metabase API
    pub async fn health_check(&self) -> Result<HealthStatus> {
        // Use ServiceManager for health check
        self.service_manager
            .auth_service()
            .ok_or_else(|| Error::Config("Auth service not available".to_string()))?
            .health_check()
            .await
            .map_err(|e| Error::Config(format!("Service error: {}", e)))
    }

    /// Gets the current authenticated user
    pub async fn get_current_user(&self) -> Result<User> {
        if !self.is_authenticated() {
            return Err(Error::Authentication("Not authenticated".to_string()));
        }

        let session_id = self.auth_manager.get_session_id()
            .ok_or_else(|| Error::Authentication("No session available".to_string()))?;
        
        // Use ServiceManager for getting current user
        self.service_manager
            .auth_service()
            .ok_or_else(|| Error::Config("Auth service not available".to_string()))?
            .get_current_user(&session_id)
            .await
            .map_err(|e| Error::Config(format!("Service error: {}", e)))
    }

    // ==================== Card Operations ====================

    /// Gets a card by ID
    pub async fn get_card(&self, id: i64) -> Result<Card> {
        #[cfg(feature = "cache")]
        {
            let cache_key = cache_key("card", id);
            if let Some(card) = self.cache.get_metadata::<Card>(&cache_key) {
                return Ok(card);
            }
        }

        // Use ServiceManager for layered architecture
        let card = self.service_manager
            .card_service()
            .ok_or_else(|| Error::Config("Card service not available".to_string()))?
            .get_card(CardId(id as i32))
            .await
            .map_err(|e| Error::Config(format!("Service error: {}", e)))?;

        #[cfg(feature = "cache")]
        {
            let cache_key = cache_key("card", id);
            let _ = self.cache.set_metadata(cache_key, &card);
        }

        Ok(card)
    }

    /// Lists all cards
    pub async fn list_cards(&self, params: Option<CardListParams>) -> Result<Vec<Card>> {
        use crate::repository::card::CardFilterParams;
        
        // Convert CardListParams to CardFilterParams and PaginationParams
        let filters = params.map(|p| CardFilterParams {
            f: p.f,
            model_type: p.model_type,
            archived: None,
            collection_id: None,
        });
        
        // Use ServiceManager for layered architecture
        self.service_manager
            .card_service()
            .ok_or_else(|| Error::Config("Card service not available".to_string()))?
            .list_cards(None, filters)
            .await
            .map_err(|e| Error::Config(format!("Service error: {}", e)))
    }

    /// Creates a new card
    pub async fn create_card(&self, card: Card) -> Result<Card> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to create card".to_string(),
            ));
        }
        
        // Use ServiceManager for layered architecture with validation
        self.service_manager
            .card_service()
            .ok_or_else(|| Error::Config("Card service not available".to_string()))?
            .create_card(card)
            .await
            .map_err(|e| Error::Config(format!("Service error: {}", e)))
    }

    /// Updates an existing card
    pub async fn update_card(&self, id: i64, updates: serde_json::Value) -> Result<Card> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to update card".to_string(),
            ));
        }

        #[cfg(feature = "cache")]
        {
            let cache_key = cache_key("card", id);
            self.cache.invalidate(&cache_key);
        }

        // Parse updates JSON into Card struct for service layer
        let card: Card = serde_json::from_value(updates)
            .map_err(|e| Error::Validation(format!("Invalid card data: {}", e)))?;
        
        // Use ServiceManager for layered architecture
        self.service_manager
            .card_service()
            .ok_or_else(|| Error::Config("Card service not available".to_string()))?
            .update_card(CardId(id as i32), card)
            .await
            .map_err(|e| Error::Config(format!("Service error: {}", e)))
    }

    /// Deletes a card
    pub async fn delete_card(&self, id: i64) -> Result<()> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to delete card".to_string(),
            ));
        }

        #[cfg(feature = "cache")]
        {
            let cache_key = cache_key("card", id);
            self.cache.invalidate(&cache_key);
        }

        // Use ServiceManager for layered architecture
        self.service_manager
            .card_service()
            .ok_or_else(|| Error::Config("Card service not available".to_string()))?
            .delete_card(CardId(id as i32))
            .await
            .map_err(|e| Error::Config(format!("Service error: {}", e)))
    }

    // ==================== Collection Operations ====================

    /// Gets a collection by ID
    pub async fn get_collection(&self, id: MetabaseId) -> Result<Collection> {
        #[cfg(feature = "cache")]
        {
            let cache_key = cache_key("collection", id.0);
            if let Some(collection) = self.cache.get_metadata::<Collection>(&cache_key) {
                return Ok(collection);
            }
        }

        let path = format!("/api/collection/{}", id.0);
        let collection: Collection = self.http_client.get(&path).await?;

        #[cfg(feature = "cache")]
        {
            let cache_key = cache_key("collection", id.0);
            let _ = self.cache.set_metadata(cache_key, &collection);
        }

        Ok(collection)
    }

    /// Lists all collections
    pub async fn list_collections(&self) -> Result<Vec<Collection>> {
        self.http_client.get("/api/collection").await
    }

    /// Creates a new collection
    pub async fn create_collection(&self, collection: Collection) -> Result<Collection> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to create collection".to_string(),
            ));
        }
        self.http_client.post("/api/collection", &collection).await
    }

    /// Updates an existing collection
    pub async fn update_collection(
        &self,
        id: MetabaseId,
        updates: serde_json::Value,
    ) -> Result<Collection> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to update collection".to_string(),
            ));
        }

        #[cfg(feature = "cache")]
        {
            let cache_key = cache_key("collection", id.0);
            self.cache.invalidate(&cache_key);
        }

        let path = format!("/api/collection/{}", id.0);
        self.http_client.put(&path, &updates).await
    }

    /// Archives a collection (Metabase doesn't delete, only archives)
    pub async fn archive_collection(&self, id: MetabaseId) -> Result<Collection> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to archive collection".to_string(),
            ));
        }

        #[cfg(feature = "cache")]
        {
            let cache_key = cache_key("collection", id.0);
            self.cache.invalidate(&cache_key);
        }

        let path = format!("/api/collection/{}", id.0);
        let updates = json!({ "archived": true });
        self.http_client.put(&path, &updates).await
    }

    // ==================== Dashboard Operations ====================

    /// Gets a dashboard by ID
    pub async fn get_dashboard(&self, id: MetabaseId) -> Result<Dashboard> {
        #[cfg(feature = "cache")]
        {
            let cache_key = cache_key("dashboard", id.0);
            if let Some(dashboard) = self.cache.get_metadata::<Dashboard>(&cache_key) {
                return Ok(dashboard);
            }
        }

        let path = format!("/api/dashboard/{}", id.0);
        let dashboard: Dashboard = self.http_client.get(&path).await?;

        #[cfg(feature = "cache")]
        {
            let cache_key = cache_key("dashboard", id.0);
            let _ = self.cache.set_metadata(cache_key, &dashboard);
        }

        Ok(dashboard)
    }

    /// Lists all dashboards
    pub async fn list_dashboards(&self, pagination: Option<Pagination>) -> Result<Vec<Dashboard>> {
        let path = if let Some(p) = pagination {
            format!("/api/dashboard?limit={}&offset={}", p.limit(), p.offset())
        } else {
            "/api/dashboard".to_string()
        };
        self.http_client.get(&path).await
    }

    /// Creates a new dashboard
    pub async fn create_dashboard(&self, dashboard: Dashboard) -> Result<Dashboard> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to create dashboard".to_string(),
            ));
        }
        self.http_client.post("/api/dashboard", &dashboard).await
    }

    /// Updates an existing dashboard
    pub async fn update_dashboard(
        &self,
        id: MetabaseId,
        updates: serde_json::Value,
    ) -> Result<Dashboard> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to update dashboard".to_string(),
            ));
        }

        #[cfg(feature = "cache")]
        {
            let cache_key = cache_key("dashboard", id.0);
            self.cache.invalidate(&cache_key);
        }

        let path = format!("/api/dashboard/{}", id.0);
        self.http_client.put(&path, &updates).await
    }

    /// Deletes a dashboard
    pub async fn delete_dashboard(&self, id: MetabaseId) -> Result<()> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to delete dashboard".to_string(),
            ));
        }

        #[cfg(feature = "cache")]
        {
            let cache_key = cache_key("dashboard", id.0);
            self.cache.invalidate(&cache_key);
        }

        let path = format!("/api/dashboard/{}", id.0);
        self.http_client.delete(&path).await
    }

    // ==================== Query Operations ====================

    /// Executes a dataset query
    pub async fn execute_query(&self, query: DatasetQuery) -> Result<QueryResult> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to execute query".to_string(),
            ));
        }
        let request = json!({
            "database": query.database.0,
            "type": query.query_type,
            "query": query.query,
            "parameters": query.parameters
        });
        self.http_client.post("/api/dataset", &request).await
    }

    /// Executes a native SQL query
    pub async fn execute_native_query(
        &self,
        database: MetabaseId,
        native_query: NativeQuery,
    ) -> Result<QueryResult> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to execute native query".to_string(),
            ));
        }
        let request = json!({
            "database": database.0,
            "type": "native",
            "native": {
                "query": native_query.query,
                "template-tags": native_query.template_tags
            }
        });
        self.http_client.post("/api/dataset", &request).await
    }

    // ==================== Extended Card Operations ====================

    /// Execute a card's query with optional parameters
    pub async fn execute_card_query(
        &self,
        card_id: i64,
        parameters: Option<Value>,
    ) -> Result<QueryResult> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to execute card query".to_string(),
            ));
        }

        // Use ServiceManager for layered architecture
        self.service_manager
            .card_service()
            .ok_or_else(|| Error::Config("Card service not available".to_string()))?
            .execute_card_query(CardId(card_id as i32), parameters)
            .await
            .map_err(|e| Error::Config(format!("Service error: {}", e)))
    }

    /// Export card query results in specified format
    pub async fn export_card_query(
        &self,
        card_id: i64,
        format: ExportFormat,
        parameters: Option<Value>,
    ) -> Result<Vec<u8>> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to export card query".to_string(),
            ));
        }

        // Use ServiceManager for layered architecture
        self.service_manager
            .card_service()
            .ok_or_else(|| Error::Config("Card service not available".to_string()))?
            .export_card_query(CardId(card_id as i32), format, parameters)
            .await
            .map_err(|e| Error::Config(format!("Service error: {}", e)))
    }

    /// Execute a pivot query for a card
    pub async fn execute_card_pivot_query(
        &self,
        card_id: i64,
        parameters: Option<Value>,
    ) -> Result<QueryResult> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to execute pivot query".to_string(),
            ));
        }

        // Use ServiceManager for layered architecture
        self.service_manager
            .card_service()
            .ok_or_else(|| Error::Config("Card service not available".to_string()))?
            .execute_card_pivot_query(CardId(card_id as i32), parameters)
            .await
            .map_err(|e| Error::Config(format!("Service error: {}", e)))
    }

    // ==================== Database Metadata Operations ====================

    /// Get database metadata including tables and fields
    pub async fn get_database_metadata(&self, database_id: MetabaseId) -> Result<DatabaseMetadata> {
        #[cfg(feature = "cache")]
        {
            let cache_key = cache_key("database_metadata", database_id.0);
            if let Some(metadata) = self.cache.get_metadata::<DatabaseMetadata>(&cache_key) {
                return Ok(metadata);
            }
        }

        let path = format!("/api/database/{}/metadata", database_id.0);
        let metadata: DatabaseMetadata = self.http_client.get(&path).await?;

        #[cfg(feature = "cache")]
        {
            let cache_key = cache_key("database_metadata", database_id.0);
            let _ = self.cache.set_metadata(cache_key, &metadata);
        }

        Ok(metadata)
    }

    /// Sync database schema
    pub async fn sync_database_schema(&self, database_id: MetabaseId) -> Result<SyncResult> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to sync database schema".to_string(),
            ));
        }

        #[cfg(feature = "cache")]
        {
            let cache_key = cache_key("database_metadata", database_id.0);
            self.cache.invalidate(&cache_key);
        }

        let path = format!("/api/database/{}/sync_schema", database_id.0);
        self.http_client.post(&path, &json!({})).await
    }

    /// Get all fields for a database
    pub async fn get_database_fields(&self, database_id: MetabaseId) -> Result<Vec<Field>> {
        let path = format!("/api/database/{}/fields", database_id.0);
        self.http_client.get(&path).await
    }

    /// Get all schemas for a database
    pub async fn get_database_schemas(&self, database_id: MetabaseId) -> Result<Vec<String>> {
        let path = format!("/api/database/{}/schemas", database_id.0);
        self.http_client.get(&path).await
    }

    // ==================== Dataset Operations ====================

    /// Execute a dataset query with advanced options
    pub async fn execute_dataset_query(&self, query: Value) -> Result<QueryResult> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to execute dataset query".to_string(),
            ));
        }

        self.http_client.post("/api/dataset", &query).await
    }

    /// Execute a native query through the dataset endpoint
    pub async fn execute_dataset_native(&self, query: Value) -> Result<QueryResult> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to execute native dataset query".to_string(),
            ));
        }

        self.http_client.post("/api/dataset/native", &query).await
    }

    /// Execute a pivot dataset query
    pub async fn execute_dataset_pivot(&self, query: Value) -> Result<QueryResult> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to execute pivot dataset query".to_string(),
            ));
        }

        self.http_client.post("/api/dataset/pivot", &query).await
    }

    /// Export dataset query results
    pub async fn export_dataset(&self, format: ExportFormat, query: Value) -> Result<Vec<u8>> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to export dataset".to_string(),
            ));
        }

        let path = format!("/api/dataset/{}", format.as_str());
        self.http_client.post_binary(&path, &query).await
    }

    // ==================== MBQL Query Operations ====================

    /// Execute an MBQL query
    #[cfg(feature = "query-builder")]
    pub async fn execute_mbql_query(
        &self,
        database_id: MetabaseId,
        query: MbqlQuery,
    ) -> Result<QueryResult> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to execute MBQL query".to_string(),
            ));
        }

        let dataset_query = query.to_dataset_query(database_id);
        self.http_client.post("/api/dataset", &dataset_query).await
    }

    /// Export MBQL query results in specified format
    #[cfg(feature = "query-builder")]
    pub async fn export_mbql_query(
        &self,
        database_id: MetabaseId,
        query: MbqlQuery,
        format: ExportFormat,
    ) -> Result<Vec<u8>> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to export MBQL query".to_string(),
            ));
        }

        let dataset_query = query.to_dataset_query(database_id);
        let path = format!("/api/dataset/{}", format.as_str());
        self.http_client.post_binary(&path, &dataset_query).await
    }

    // ==================== SQL Convenience Methods ====================

    /// Execute a simple SQL query
    pub async fn execute_sql(&self, database_id: MetabaseId, sql: &str) -> Result<QueryResult> {
        let native_query = NativeQuery::new(sql);
        self.execute_native_query(database_id, native_query).await
    }

    /// Execute a parameterized SQL query
    pub async fn execute_sql_with_params(
        &self,
        database_id: MetabaseId,
        sql: &str,
        params: HashMap<String, Value>,
    ) -> Result<QueryResult> {
        let mut native_query = NativeQuery::new(sql);
        for (name, value) in params {
            native_query = native_query.with_param(&name, value);
        }
        self.execute_native_query(database_id, native_query).await
    }

    /// Export SQL query results in specified format
    pub async fn export_sql_query(
        &self,
        database_id: MetabaseId,
        sql: &str,
        format: ExportFormat,
    ) -> Result<Vec<u8>> {
        if !self.is_authenticated() {
            return Err(Error::Authentication(
                "Authentication required to export SQL query".to_string(),
            ));
        }

        let native_query = NativeQuery::new(sql);
        let request = json!({
            "database": database_id.0,
            "type": "native",
            "native": {
                "query": native_query.query,
                "template-tags": native_query.template_tags
            }
        });

        let path = format!("/api/dataset/{}", format.as_str());
        self.http_client.post_binary(&path, &request).await
    }
}
