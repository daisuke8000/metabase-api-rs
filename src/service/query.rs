//! Query service implementation
//!
//! This module provides business logic for Query operations.

use super::traits::{Service, ServiceError, ServiceResult};
use crate::core::models::query::{NativeQuery, QueryResult};
use crate::repository::query::QueryRepository;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

/// Service trait for Query operations
#[async_trait]
pub trait QueryService: Service {
    /// Execute a native SQL query
    async fn execute_native_query(
        &self,
        database_id: i32,
        query: NativeQuery,
    ) -> ServiceResult<QueryResult>;

    /// Execute a SQL query with parameters
    async fn execute_sql_with_params(
        &self,
        database_id: i32,
        sql: &str,
        params: HashMap<String, serde_json::Value>,
    ) -> ServiceResult<QueryResult>;

    /// Execute a simple SQL query
    async fn execute_sql(&self, database_id: i32, sql: &str) -> ServiceResult<QueryResult>;

    /// Validate SQL query syntax (basic validation)
    async fn validate_query(&self, sql: &str) -> ServiceResult<()>;
}

/// HTTP implementation of QueryService
pub struct HttpQueryService {
    repository: Arc<dyn QueryRepository>,
}

impl HttpQueryService {
    /// Create a new HTTP query service
    pub fn new(repository: Arc<dyn QueryRepository>) -> Self {
        Self { repository }
    }

    /// Validate SQL query
    fn validate_sql(&self, sql: &str) -> ServiceResult<()> {
        // Basic SQL validation
        if sql.trim().is_empty() {
            return Err(ServiceError::Validation(
                "SQL query cannot be empty".to_string(),
            ));
        }

        // Check for dangerous operations (basic check)
        let sql_upper = sql.to_uppercase();
        let dangerous_keywords = [
            "DROP", "DELETE", "TRUNCATE", "ALTER", "CREATE", "GRANT", "REVOKE",
        ];

        for keyword in &dangerous_keywords {
            if sql_upper.contains(keyword) {
                // Allow if it's in a comment
                if !sql_upper.contains(&format!("--{}", keyword))
                    && !sql_upper.contains(&format!("/*{}*/", keyword))
                {
                    return Err(ServiceError::BusinessRule(format!(
                        "Query contains potentially dangerous operation: {}",
                        keyword
                    )));
                }
            }
        }

        Ok(())
    }
}

#[async_trait]
impl Service for HttpQueryService {
    fn name(&self) -> &str {
        "QueryService"
    }
}

#[async_trait]
impl QueryService for HttpQueryService {
    async fn execute_native_query(
        &self,
        database_id: i32,
        query: NativeQuery,
    ) -> ServiceResult<QueryResult> {
        // Validate the query
        self.validate_sql(&query.query)?;

        // Execute via repository
        self.repository
            .execute_native_query(database_id, query)
            .await
            .map_err(ServiceError::from)
    }

    async fn execute_sql_with_params(
        &self,
        database_id: i32,
        sql: &str,
        params: HashMap<String, serde_json::Value>,
    ) -> ServiceResult<QueryResult> {
        // Validate the query
        self.validate_sql(sql)?;

        // Build native query
        let query = NativeQuery::builder(sql).with_params(params).build();

        // Execute via repository
        self.repository
            .execute_native_query(database_id, query)
            .await
            .map_err(ServiceError::from)
    }

    async fn execute_sql(&self, database_id: i32, sql: &str) -> ServiceResult<QueryResult> {
        // Validate the query
        self.validate_sql(sql)?;

        // Build native query
        let query = NativeQuery::builder(sql).build();

        // Execute via repository
        self.repository
            .execute_native_query(database_id, query)
            .await
            .map_err(ServiceError::from)
    }

    async fn validate_query(&self, sql: &str) -> ServiceResult<()> {
        self.validate_sql(sql)
    }
}
