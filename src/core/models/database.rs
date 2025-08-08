//! Database model representing Metabase database connections
//!
//! This module provides the core data structures for working with
//! Metabase database connections, including tables and fields.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::MetabaseId;

/// Connection source for database
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionSource {
    #[default]
    Admin,
    Setup,
}

/// Unique identifier for a database field
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FieldId(pub i64);

/// Unique identifier for a database table
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TableId(pub i64);

/// Represents a Metabase database connection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Database {
    /// Unique identifier for the database
    pub id: MetabaseId,

    /// Database name
    pub name: String,

    /// Database engine (e.g., "postgres", "mysql", "h2")
    pub engine: String,

    /// Connection details (host, port, database name, etc.)
    pub details: Value,

    /// Whether full sync is enabled
    #[serde(default)]
    pub is_full_sync: bool,

    /// Whether on-demand sync is enabled
    #[serde(default)]
    pub is_on_demand: bool,

    /// Whether the database is a sample database
    #[serde(default)]
    pub is_sample: bool,

    /// Cache field values for this database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_field_values_schedule: Option<String>,

    /// Metadata sync schedule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_sync_schedule: Option<String>,

    /// When the database was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// When the database was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Represents a table in a database
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatabaseTable {
    /// Unique identifier for the table
    pub id: TableId,

    /// Database ID this table belongs to
    pub db_id: MetabaseId,

    /// Table name
    pub name: String,

    /// Database schema name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

    /// Display name for the table
    pub display_name: String,

    /// Whether the table is active
    #[serde(default = "default_true")]
    pub active: bool,

    /// Table description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Entity type (e.g., "entity/GenericTable", "entity/EventTable")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,

    /// Visibility type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_type: Option<String>,

    /// When the table was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// When the table was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Represents a field in a database table
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatabaseField {
    /// Unique identifier for the field
    pub id: FieldId,

    /// Table ID this field belongs to
    pub table_id: TableId,

    /// Field name
    pub name: String,

    /// Display name for the field
    pub display_name: String,

    /// Database-specific type
    pub database_type: String,

    /// Base type (e.g., "type/Text", "type/Integer")
    pub base_type: String,

    /// Semantic type (e.g., "type/Email", "type/URL")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_type: Option<String>,

    /// Whether this field is active
    #[serde(default = "default_true")]
    pub active: bool,

    /// Field description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether this is a primary key
    #[serde(default)]
    pub is_pk: bool,

    /// Foreign key target field ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fk_target_field_id: Option<FieldId>,

    /// Field position in the table
    #[serde(default)]
    pub position: i32,

    /// Visibility type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_type: Option<String>,

    /// When the field was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// When the field was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Request to create a new database connection
#[derive(Debug, Clone, Serialize)]
pub struct CreateDatabaseRequest {
    /// Database name
    pub name: String,

    /// Database engine
    pub engine: String,

    /// Connection details
    pub details: Value,

    /// Whether to enable full sync
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_full_sync: Option<bool>,

    /// Whether to enable on-demand sync
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_on_demand: Option<bool>,

    /// Schedule for caching field values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_field_values_schedule: Option<String>,

    /// Schedule for metadata sync
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_sync_schedule: Option<String>,
}

/// Request to update a database connection
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateDatabaseRequest {
    /// New name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// New connection details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,

    /// Whether to enable full sync
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_full_sync: Option<bool>,

    /// Whether to enable on-demand sync
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_on_demand: Option<bool>,

    /// Schedule for caching field values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_field_values_schedule: Option<String>,

    /// Schedule for metadata sync
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_sync_schedule: Option<String>,
}

/// Database sync status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatabaseSyncStatus {
    /// Current sync status
    pub status: String,

    /// Sync progress (0-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f32>,

    /// Error message if sync failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// When the sync started
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,

    /// When the sync completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
}

fn default_true() -> bool {
    true
}

impl Database {
    /// Creates a new database builder
    pub fn builder(name: impl Into<String>, engine: impl Into<String>) -> DatabaseBuilder {
        DatabaseBuilder::new(name, engine)
    }
}

/// Builder for creating Database instances
pub struct DatabaseBuilder {
    name: String,
    engine: String,
    details: Value,
    is_full_sync: bool,
    is_on_demand: bool,
    cache_field_values_schedule: Option<String>,
    metadata_sync_schedule: Option<String>,
}

impl DatabaseBuilder {
    /// Creates a new database builder
    pub fn new(name: impl Into<String>, engine: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            engine: engine.into(),
            details: Value::Object(serde_json::Map::new()),
            is_full_sync: true,
            is_on_demand: false,
            cache_field_values_schedule: None,
            metadata_sync_schedule: None,
        }
    }

    /// Sets the connection details
    pub fn details(mut self, details: Value) -> Self {
        self.details = details;
        self
    }

    /// Sets whether full sync is enabled
    pub fn full_sync(mut self, enabled: bool) -> Self {
        self.is_full_sync = enabled;
        self
    }

    /// Sets whether on-demand sync is enabled
    pub fn on_demand_sync(mut self, enabled: bool) -> Self {
        self.is_on_demand = enabled;
        self
    }

    /// Sets the cache field values schedule
    pub fn cache_schedule(mut self, schedule: impl Into<String>) -> Self {
        self.cache_field_values_schedule = Some(schedule.into());
        self
    }

    /// Sets the metadata sync schedule
    pub fn sync_schedule(mut self, schedule: impl Into<String>) -> Self {
        self.metadata_sync_schedule = Some(schedule.into());
        self
    }

    /// Builds the Database instance
    pub fn build(self) -> Database {
        Database {
            id: MetabaseId(0), // Will be set by the server
            name: self.name,
            engine: self.engine,
            details: self.details,
            is_full_sync: self.is_full_sync,
            is_on_demand: self.is_on_demand,
            is_sample: false,
            cache_field_values_schedule: self.cache_field_values_schedule,
            metadata_sync_schedule: self.metadata_sync_schedule,
            created_at: None,
            updated_at: None,
        }
    }

    /// Builds a CreateDatabaseRequest
    pub fn build_request(self) -> CreateDatabaseRequest {
        CreateDatabaseRequest {
            name: self.name,
            engine: self.engine,
            details: self.details,
            is_full_sync: Some(self.is_full_sync),
            is_on_demand: Some(self.is_on_demand),
            cache_field_values_schedule: self.cache_field_values_schedule,
            metadata_sync_schedule: self.metadata_sync_schedule,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_database_creation() {
        let database = Database::builder("Test DB", "postgres")
            .details(json!({
                "host": "localhost",
                "port": 5432,
                "dbname": "testdb",
                "user": "testuser"
            }))
            .full_sync(true)
            .on_demand_sync(false)
            .build();

        assert_eq!(database.name, "Test DB");
        assert_eq!(database.engine, "postgres");
        assert!(database.is_full_sync);
        assert!(!database.is_on_demand);
    }

    #[test]
    fn test_database_table() {
        let table = DatabaseTable {
            id: TableId(1),
            db_id: MetabaseId(1),
            name: "users".to_string(),
            schema: Some("public".to_string()),
            display_name: "Users".to_string(),
            active: true,
            description: Some("User accounts".to_string()),
            entity_type: Some("entity/UserTable".to_string()),
            visibility_type: None,
            created_at: None,
            updated_at: None,
        };

        assert_eq!(table.name, "users");
        assert_eq!(table.display_name, "Users");
        assert!(table.active);
    }

    #[test]
    fn test_database_field() {
        let field = DatabaseField {
            id: FieldId(1),
            table_id: TableId(1),
            name: "email".to_string(),
            display_name: "Email".to_string(),
            database_type: "VARCHAR(255)".to_string(),
            base_type: "type/Text".to_string(),
            semantic_type: Some("type/Email".to_string()),
            active: true,
            description: None,
            is_pk: false,
            fk_target_field_id: None,
            position: 2,
            visibility_type: None,
            created_at: None,
            updated_at: None,
        };

        assert_eq!(field.name, "email");
        assert_eq!(field.base_type, "type/Text");
        assert_eq!(field.semantic_type, Some("type/Email".to_string()));
        assert!(!field.is_pk);
    }

    #[test]
    fn test_create_database_request() {
        let request = Database::builder("Production DB", "mysql")
            .details(json!({
                "host": "db.example.com",
                "port": 3306,
                "dbname": "production"
            }))
            .cache_schedule("0 0 * * *")
            .build_request();

        assert_eq!(request.name, "Production DB");
        assert_eq!(request.engine, "mysql");
        assert_eq!(request.is_full_sync, Some(true));
        assert_eq!(
            request.cache_field_values_schedule,
            Some("0 0 * * *".to_string())
        );
    }

    #[test]
    fn test_update_database_request() {
        let request = UpdateDatabaseRequest {
            name: Some("Updated DB".to_string()),
            is_full_sync: Some(false),
            ..Default::default()
        };

        assert_eq!(request.name, Some("Updated DB".to_string()));
        assert_eq!(request.is_full_sync, Some(false));
        assert!(request.details.is_none());
    }
}

// ==================== Database Metadata Models ====================

/// Database metadata including tables and fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseMetadata {
    /// Database ID
    pub id: MetabaseId,

    /// Database name
    pub name: String,

    /// Database engine
    pub engine: String,

    /// List of tables in the database
    pub tables: Vec<TableMetadata>,

    /// Database features
    #[serde(default)]
    pub features: Vec<String>,

    /// Native query permissions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_permissions: Option<String>,
}

/// Table metadata including fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableMetadata {
    /// Table ID
    pub id: TableId,

    /// Table name
    pub name: String,

    /// Table schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

    /// Display name
    pub display_name: String,

    /// Table description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Entity type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,

    /// List of fields in the table
    pub fields: Vec<FieldMetadata>,

    /// Whether the table is active
    #[serde(default = "default_true")]
    pub active: bool,
}

/// Field metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldMetadata {
    /// Field ID
    pub id: FieldId,

    /// Field name
    pub name: String,

    /// Display name
    pub display_name: String,

    /// Database type
    pub database_type: String,

    /// Base type (Metabase type)
    pub base_type: String,

    /// Semantic type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_type: Option<String>,

    /// Field description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether this is a primary key
    #[serde(default)]
    pub is_pk: bool,

    /// Foreign key target field ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fk_target_field_id: Option<FieldId>,

    /// Field position in the table
    pub position: i32,

    /// Whether the field is active
    #[serde(default = "default_true")]
    pub active: bool,
}

/// Database sync result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    /// Sync task ID
    pub id: String,

    /// Sync status
    pub status: String,

    /// Status message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// When the sync started
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,

    /// When the sync completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
}
