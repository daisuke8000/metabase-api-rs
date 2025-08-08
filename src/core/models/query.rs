//! Query models for executing queries against Metabase
//!
//! This module provides data structures for dataset queries,
//! native SQL queries, and their results.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::common::MetabaseId;

/// Represents a dataset query to be executed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatasetQuery {
    /// Database ID to query against
    pub database: MetabaseId,

    /// Query type (e.g., "query", "native")
    #[serde(rename = "type")]
    pub query_type: String,

    /// The actual query (MBQL or native)
    pub query: Value,

    /// Optional parameters for the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<QueryParameter>>,

    /// Optional constraints (e.g., max rows)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<QueryConstraints>,
}

/// Represents a native SQL query
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NativeQuery {
    /// The SQL query string
    pub query: String,

    /// Template tags for parameterized queries
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    #[serde(rename = "template-tags")]
    pub template_tags: HashMap<String, TemplateTag>,

    /// Collection of tables used in the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<String>,
}

/// Parameter for a query
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryParameter {
    /// Parameter ID or name
    pub id: String,

    /// Parameter type
    #[serde(rename = "type")]
    pub parameter_type: String,

    /// Parameter value
    pub value: Value,

    /// Target field or variable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Value>,
}

/// Template tag for native queries
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateTag {
    /// Unique identifier for the tag
    pub id: String,

    /// Tag name
    pub name: String,

    /// Display name
    #[serde(rename = "display-name")]
    pub display_name: String,

    /// Tag type (e.g., "text", "number", "date")
    #[serde(rename = "type")]
    pub tag_type: String,

    /// Whether the tag is required
    #[serde(default)]
    pub required: bool,

    /// Default value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
}

/// Query constraints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryConstraints {
    /// Maximum number of rows to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,

    /// Maximum execution time in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_execution_time: Option<i32>,
}

/// Result of a query execution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryResult {
    /// The actual data returned
    pub data: QueryData,

    /// Database that was queried
    pub database_id: MetabaseId,

    /// When the query started
    pub started_at: DateTime<Utc>,

    /// When the query finished
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<DateTime<Utc>>,

    /// The query that was executed
    pub json_query: Value,

    /// Query status
    pub status: QueryStatus,

    /// Row count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i32>,

    /// Running time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_time: Option<i32>,
}

/// Query execution status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QueryStatus {
    /// Query is still running
    Running,
    /// Query completed successfully
    Completed,
    /// Query failed with an error
    Failed,
    /// Query was cancelled
    Cancelled,
}

/// The actual data from a query result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryData {
    /// Column information
    pub cols: Vec<Column>,

    /// Row data
    pub rows: Vec<Vec<Value>>,

    /// Native form of the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_form: Option<Value>,

    /// Insights from the query
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insights: Vec<Insight>,
}

/// Column metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Column {
    /// Column name
    pub name: String,

    /// Display name
    pub display_name: String,

    /// Base type (e.g., "type/Text", "type/Integer")
    pub base_type: String,

    /// Effective type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_type: Option<String>,

    /// Semantic type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_type: Option<String>,

    /// Field reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<Value>,
}

/// Query insight
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Insight {
    /// Insight type
    #[serde(rename = "type")]
    pub insight_type: String,

    /// Insight value
    pub value: Value,
}

/// Request to execute a dataset query
#[derive(Debug, Clone, Serialize)]
pub struct ExecuteQueryRequest {
    /// The dataset query to execute
    pub dataset_query: DatasetQuery,

    /// Visualization settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visualization_settings: Option<Value>,

    /// Display type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

/// Request to execute a native SQL query
#[derive(Debug, Clone, Serialize)]
pub struct ExecuteNativeQueryRequest {
    /// Database to execute against
    pub database: MetabaseId,

    /// The native query
    pub native: NativeQuery,

    /// Parameters for the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<QueryParameter>>,
}

impl DatasetQuery {
    /// Creates a new dataset query builder
    pub fn builder(database: MetabaseId) -> DatasetQueryBuilder {
        DatasetQueryBuilder::new(database)
    }
}

impl NativeQuery {
    /// Creates a new NativeQuery with the given SQL
    pub fn new(sql: impl Into<String>) -> Self {
        Self {
            query: sql.into(),
            template_tags: HashMap::new(),
            collection: None,
        }
    }

    /// Creates a new NativeQuery builder
    pub fn builder(sql: impl Into<String>) -> NativeQueryBuilder {
        NativeQueryBuilder::new(sql)
    }

    /// Add a parameter to the query
    pub fn with_param(mut self, name: &str, value: Value) -> Self {
        let tag = TemplateTag {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            display_name: name.to_string(),
            tag_type: match &value {
                Value::String(_) => "text",
                Value::Number(_) => "number",
                Value::Bool(_) => "text",
                _ => "text",
            }
            .to_string(),
            required: false,
            default: Some(value),
        };
        self.template_tags.insert(name.to_string(), tag);
        self
    }
}

/// Builder for creating DatasetQuery instances
pub struct DatasetQueryBuilder {
    database: MetabaseId,
    query_type: String,
    query: Value,
    parameters: Option<Vec<QueryParameter>>,
    constraints: Option<QueryConstraints>,
}

impl DatasetQueryBuilder {
    /// Creates a new query builder
    pub fn new(database: MetabaseId) -> Self {
        Self {
            database,
            query_type: "query".to_string(),
            query: Value::Null,
            parameters: None,
            constraints: None,
        }
    }

    /// Sets the query type
    pub fn query_type(mut self, query_type: impl Into<String>) -> Self {
        self.query_type = query_type.into();
        self
    }

    /// Sets the query
    pub fn query(mut self, query: Value) -> Self {
        self.query = query;
        self
    }

    /// Sets the parameters
    pub fn parameters(mut self, params: Vec<QueryParameter>) -> Self {
        self.parameters = Some(params);
        self
    }

    /// Sets the constraints
    pub fn constraints(mut self, constraints: QueryConstraints) -> Self {
        self.constraints = Some(constraints);
        self
    }

    /// Builds the DatasetQuery
    pub fn build(self) -> DatasetQuery {
        DatasetQuery {
            database: self.database,
            query_type: self.query_type,
            query: self.query,
            parameters: self.parameters,
            constraints: self.constraints,
        }
    }
}

/// Builder for creating NativeQuery instances
pub struct NativeQueryBuilder {
    query: String,
    template_tags: HashMap<String, TemplateTag>,
    collection: Option<String>,
}

impl NativeQueryBuilder {
    /// Creates a new NativeQuery builder
    pub fn new(sql: impl Into<String>) -> Self {
        Self {
            query: sql.into(),
            template_tags: HashMap::new(),
            collection: None,
        }
    }

    /// Adds a generic parameter
    pub fn add_param(mut self, name: &str, param_type: &str, value: Value) -> Self {
        let tag = TemplateTag {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            display_name: name.to_string(),
            tag_type: param_type.to_string(),
            required: false,
            default: Some(value),
        };
        self.template_tags.insert(name.to_string(), tag);
        self
    }

    /// Adds a text parameter
    pub fn add_text_param(self, name: &str, value: &str) -> Self {
        self.add_param(name, "text", Value::String(value.to_string()))
    }

    /// Adds a number parameter
    pub fn add_number_param(self, name: &str, value: f64) -> Self {
        self.add_param(name, "number", serde_json::json!(value))
    }

    /// Adds a date parameter
    pub fn add_date_param(self, name: &str, value: &str) -> Self {
        self.add_param(name, "date", Value::String(value.to_string()))
    }

    /// Sets the collection
    pub fn collection(mut self, collection: impl Into<String>) -> Self {
        self.collection = Some(collection.into());
        self
    }

    /// Builds the NativeQuery
    pub fn build(self) -> NativeQuery {
        NativeQuery {
            query: self.query,
            template_tags: self.template_tags,
            collection: self.collection,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_dataset_query_builder() {
        let query = DatasetQuery::builder(MetabaseId(1))
            .query_type("native")
            .query(json!({"query": "SELECT * FROM users"}))
            .build();

        assert_eq!(query.database, MetabaseId(1));
        assert_eq!(query.query_type, "native");
        assert_eq!(query.query, json!({"query": "SELECT * FROM users"}));
    }

    #[test]
    fn test_native_query() {
        let mut template_tags = HashMap::new();
        template_tags.insert(
            "date".to_string(),
            TemplateTag {
                id: "test-id".to_string(),
                name: "date".to_string(),
                display_name: "Date".to_string(),
                tag_type: "date".to_string(),
                required: true,
                default: None,
            },
        );

        let native = NativeQuery {
            query: "SELECT * FROM orders WHERE created_at > {{date}}".to_string(),
            template_tags,
            collection: None,
        };

        assert_eq!(native.template_tags.len(), 1);
        assert!(native.template_tags.contains_key("date"));
        assert!(native.template_tags["date"].required);
    }

    #[test]
    fn test_native_query_builder() {
        let query = NativeQuery::builder("SELECT * FROM orders WHERE status = {{status}}")
            .add_text_param("status", "completed")
            .build();

        assert_eq!(
            query.query,
            "SELECT * FROM orders WHERE status = {{status}}"
        );
        assert!(query.template_tags.contains_key("status"));
        assert_eq!(query.template_tags["status"].tag_type, "text");
        assert_eq!(
            query.template_tags["status"].default,
            Some(json!("completed"))
        );
    }

    #[test]
    fn test_query_result() {
        let result = QueryResult {
            data: QueryData {
                cols: vec![Column {
                    name: "id".to_string(),
                    display_name: "ID".to_string(),
                    base_type: "type/Integer".to_string(),
                    effective_type: None,
                    semantic_type: None,
                    field_ref: None,
                }],
                rows: vec![vec![json!(1)], vec![json!(2)]],
                native_form: None,
                insights: vec![],
            },
            database_id: MetabaseId(1),
            started_at: Utc::now(),
            finished_at: Some(Utc::now()),
            json_query: json!({}),
            status: QueryStatus::Completed,
            row_count: Some(2),
            running_time: Some(150),
        };

        assert_eq!(result.status, QueryStatus::Completed);
        assert_eq!(result.row_count, Some(2));
        assert_eq!(result.data.rows.len(), 2);
    }

    #[test]
    fn test_query_constraints() {
        let constraints = QueryConstraints {
            max_results: Some(1000),
            max_execution_time: Some(60),
        };

        assert_eq!(constraints.max_results, Some(1000));
        assert_eq!(constraints.max_execution_time, Some(60));
    }
}
