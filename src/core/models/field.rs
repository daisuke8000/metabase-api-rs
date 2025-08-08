//! Field model representing database fields in Metabase

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::database::{FieldId, TableId};

/// Represents a database field with full metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    /// Field ID
    pub id: FieldId,

    /// Table ID this field belongs to
    pub table_id: TableId,

    /// Field name in the database
    pub name: String,

    /// Display name for UI
    pub display_name: String,

    /// Database type (e.g., "VARCHAR(255)", "INTEGER")
    pub database_type: String,

    /// Metabase base type (e.g., "type/Text", "type/Integer")
    pub base_type: String,

    /// Semantic type (e.g., "type/Email", "type/Category")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_type: Option<String>,

    /// Field description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether this field is active
    #[serde(default = "default_true")]
    pub active: bool,

    /// Position in the table
    pub position: i32,

    /// Whether this is a primary key
    #[serde(default)]
    pub is_pk: bool,

    /// Foreign key target field ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fk_target_field_id: Option<FieldId>,

    /// Visibility type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_type: Option<String>,

    /// Field fingerprint (contains statistics about the field)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<Value>,

    /// Whether field values should be cached
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_field_values: Option<String>,

    /// Settings for this field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Value>,

    /// When the field was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// When the field was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Field values for a specific field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValues {
    /// Field ID
    pub field_id: FieldId,

    /// List of distinct values
    pub values: Vec<Value>,

    /// Whether there are more values than returned
    #[serde(default)]
    pub has_more_values: bool,
}

/// Request to update a field
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateFieldRequest {
    /// New display name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// New description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// New semantic type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_type: Option<String>,

    /// New visibility type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_type: Option<String>,

    /// New settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Value>,

    /// Whether field values should be cached
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_field_values: Option<String>,
}

fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_field_serialization() {
        let field = Field {
            id: FieldId(1),
            table_id: TableId(10),
            name: "email".to_string(),
            display_name: "Email Address".to_string(),
            database_type: "VARCHAR(255)".to_string(),
            base_type: "type/Text".to_string(),
            semantic_type: Some("type/Email".to_string()),
            description: Some("User email address".to_string()),
            active: true,
            position: 3,
            is_pk: false,
            fk_target_field_id: None,
            visibility_type: Some("normal".to_string()),
            fingerprint: None,
            has_field_values: Some("list".to_string()),
            settings: None,
            created_at: None,
            updated_at: None,
        };

        let json = serde_json::to_value(&field).unwrap();
        assert_eq!(json["name"], "email");
        assert_eq!(json["base_type"], "type/Text");
        assert_eq!(json["semantic_type"], "type/Email");
    }

    #[test]
    fn test_field_values() {
        let values = FieldValues {
            field_id: FieldId(5),
            values: vec![json!("option1"), json!("option2"), json!("option3")],
            has_more_values: false,
        };

        assert_eq!(values.values.len(), 3);
        assert!(!values.has_more_values);
    }

    #[test]
    fn test_update_field_request() {
        let request = UpdateFieldRequest {
            display_name: Some("Updated Name".to_string()),
            semantic_type: Some("type/Category".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["display_name"], "Updated Name");
        assert_eq!(json["semantic_type"], "type/Category");
        assert_eq!(json.get("description"), None);
    }
}
