//! Query service implementation
//!
//! This module provides business logic for Query operations.

use super::traits::{Service, ServiceError, ServiceResult};
use crate::core::models::query::{NativeQuery, QueryResult};
use crate::core::models::DatasetQuery;
use crate::repository::query::QueryRepository;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Service trait for Query operations
#[async_trait]
pub trait QueryService: Service {
    /// Execute a dataset query
    async fn execute_dataset_query(&self, query: DatasetQuery) -> ServiceResult<QueryResult>;

    /// Execute a native SQL query
    async fn execute_native_query(
        &self,
        database_id: i32,
        query: NativeQuery,
    ) -> ServiceResult<QueryResult>;

    /// Execute a raw query (JSON format)
    async fn execute_raw_query(&self, query: Value) -> ServiceResult<QueryResult>;

    /// Execute a pivot query
    async fn execute_pivot_query(&self, query: Value) -> ServiceResult<QueryResult>;

    /// Execute a SQL query with parameters
    async fn execute_sql_with_params(
        &self,
        database_id: i32,
        sql: &str,
        params: HashMap<String, serde_json::Value>,
    ) -> ServiceResult<QueryResult>;

    /// Execute a simple SQL query
    async fn execute_sql(&self, database_id: i32, sql: &str) -> ServiceResult<QueryResult>;

    /// Export query results
    async fn export_query(&self, format: &str, query: Value) -> ServiceResult<Vec<u8>>;

    /// Validate SQL query syntax (basic validation)
    async fn validate_query(&self, sql: &str) -> ServiceResult<()>;

    /// Validate a dataset query
    async fn validate_dataset_query(&self, query: &DatasetQuery) -> ServiceResult<()>;
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
    async fn execute_dataset_query(&self, query: DatasetQuery) -> ServiceResult<QueryResult> {
        // Validate the dataset query
        self.validate_dataset_query(&query).await?;

        // Execute via repository
        self.repository
            .execute_dataset_query(query)
            .await
            .map_err(ServiceError::from)
    }

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

    async fn execute_raw_query(&self, query: Value) -> ServiceResult<QueryResult> {
        // Validate that query has required fields
        if !query.is_object() {
            return Err(ServiceError::Validation(
                "Query must be a JSON object".to_string(),
            ));
        }

        // Execute via repository
        self.repository
            .execute_raw_query(query)
            .await
            .map_err(ServiceError::from)
    }

    async fn execute_pivot_query(&self, query: Value) -> ServiceResult<QueryResult> {
        // Validate that query has required fields
        if !query.is_object() {
            return Err(ServiceError::Validation(
                "Query must be a JSON object".to_string(),
            ));
        }

        // Execute via repository
        self.repository
            .execute_pivot_query(query)
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

    async fn export_query(&self, format: &str, query: Value) -> ServiceResult<Vec<u8>> {
        // Validate export format
        let valid_formats = ["csv", "json", "xlsx"];
        if !valid_formats.contains(&format) {
            return Err(ServiceError::Validation(format!(
                "Invalid export format: {}. Must be one of: csv, json, xlsx",
                format
            )));
        }

        // Validate that query has required fields
        if !query.is_object() {
            return Err(ServiceError::Validation(
                "Query must be a JSON object".to_string(),
            ));
        }

        // Export via repository
        self.repository
            .export_query(format, query)
            .await
            .map_err(ServiceError::from)
    }

    async fn validate_query(&self, sql: &str) -> ServiceResult<()> {
        self.validate_sql(sql)
    }

    async fn validate_dataset_query(&self, query: &DatasetQuery) -> ServiceResult<()> {
        // Validate database ID
        if query.database.0 < 1 {
            return Err(ServiceError::Validation(
                "Invalid database ID: must be positive".to_string(),
            ));
        }

        // Validate query content
        if query.query.is_null() {
            return Err(ServiceError::Validation(
                "Query content cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::query::QueryStatus;
    use crate::core::models::MetabaseId;
    use crate::repository::query::{MockQueryRepository, QueryRepository};
    use serde_json::json;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_execute_dataset_query() {
        // Arrange
        let repository = Arc::new(MockQueryRepository::new()) as Arc<dyn QueryRepository>;
        let service = HttpQueryService::new(repository);
        let query = DatasetQuery {
            database: MetabaseId(1),
            query_type: "native".to_string(),
            query: json!({"query": "SELECT * FROM users"}),
            parameters: None,
            constraints: None,
        };

        // Act
        let result = service.execute_dataset_query(query).await;

        // Assert
        assert!(result.is_ok());
        let query_result = result.unwrap();
        assert_eq!(query_result.status, QueryStatus::Completed);
    }

    #[tokio::test]
    async fn test_validate_sql() {
        let repository = Arc::new(MockQueryRepository::new()) as Arc<dyn QueryRepository>;
        let service = HttpQueryService::new(repository);

        // Valid SQL
        assert!(service.validate_sql("SELECT * FROM users").is_ok());

        // Empty SQL
        assert!(service.validate_sql("").is_err());

        // Dangerous SQL
        assert!(service.validate_sql("DROP TABLE users").is_err());
    }

    #[tokio::test]
    async fn test_export_query() {
        let repository = Arc::new(MockQueryRepository::new()) as Arc<dyn QueryRepository>;
        let service = HttpQueryService::new(repository);
        let query = json!({
            "database": 1,
            "type": "native",
            "native": {"query": "SELECT * FROM products"}
        });

        // Valid format
        let result = service.export_query("csv", query.clone()).await;
        assert!(result.is_ok());

        // Invalid format
        let result = service.export_query("invalid", query).await;
        assert!(result.is_err());
    }
}
