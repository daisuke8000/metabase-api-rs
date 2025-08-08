//! Field reference types for MBQL queries

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Represents a field reference in MBQL
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldRef {
    /// Reference by field ID
    FieldId {
        #[serde(rename = "field-id")]
        id: i64,
    },
    /// Reference by field name
    FieldName {
        #[serde(rename = "field-literal")]
        name: String,
        #[serde(rename = "type")]
        field_type: FieldType,
    },
    /// Foreign key reference
    ForeignKey {
        #[serde(rename = "fk->")]
        source_field: Box<FieldRef>,
        target_field: Box<FieldRef>,
    },
    /// Expression reference
    Expression {
        #[serde(rename = "expression")]
        name: String,
    },
}

impl FieldRef {
    /// Create a field reference by ID
    pub fn field_id(id: i64) -> Self {
        FieldRef::FieldId { id }
    }

    /// Create a field reference by name
    pub fn field_name(name: impl Into<String>) -> Self {
        FieldRef::FieldName {
            name: name.into(),
            field_type: FieldType::Any,
        }
    }

    /// Create a field reference by name with type
    pub fn field_name_typed(name: impl Into<String>, field_type: FieldType) -> Self {
        FieldRef::FieldName {
            name: name.into(),
            field_type,
        }
    }

    /// Create a foreign key reference
    pub fn foreign_key(source: FieldRef, target: FieldRef) -> Self {
        FieldRef::ForeignKey {
            source_field: Box::new(source),
            target_field: Box::new(target),
        }
    }

    /// Create an expression reference
    pub fn expression(name: impl Into<String>) -> Self {
        FieldRef::Expression { name: name.into() }
    }

    /// Convert to JSON representation
    pub fn to_json(&self) -> Value {
        match self {
            FieldRef::FieldId { id } => json!(["field-id", id]),
            FieldRef::FieldName { name, field_type } => {
                json!(["field-literal", name, field_type.to_string()])
            }
            FieldRef::ForeignKey {
                source_field,
                target_field,
            } => {
                json!(["fk->", source_field.to_json(), target_field.to_json()])
            }
            FieldRef::Expression { name } => json!(["expression", name]),
        }
    }
}

/// Field type for field-literal references
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FieldType {
    /// Any type
    #[serde(rename = "type/*")]
    Any,
    /// Text type
    #[serde(rename = "type/Text")]
    Text,
    /// Number type
    #[serde(rename = "type/Number")]
    Number,
    /// Integer type
    #[serde(rename = "type/Integer")]
    Integer,
    /// Float type
    #[serde(rename = "type/Float")]
    Float,
    /// Boolean type
    #[serde(rename = "type/Boolean")]
    Boolean,
    /// Date type
    #[serde(rename = "type/Date")]
    Date,
    /// DateTime type
    #[serde(rename = "type/DateTime")]
    DateTime,
    /// Time type
    #[serde(rename = "type/Time")]
    Time,
}

impl FieldType {
    /// Convert to string representation
    pub fn to_string(&self) -> &'static str {
        match self {
            FieldType::Any => "type/*",
            FieldType::Text => "type/Text",
            FieldType::Number => "type/Number",
            FieldType::Integer => "type/Integer",
            FieldType::Float => "type/Float",
            FieldType::Boolean => "type/Boolean",
            FieldType::Date => "type/Date",
            FieldType::DateTime => "type/DateTime",
            FieldType::Time => "type/Time",
        }
    }
}

impl Default for FieldType {
    fn default() -> Self {
        FieldType::Any
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_id() {
        let field = FieldRef::field_id(10);
        assert_eq!(field.to_json(), json!(["field-id", 10]));
    }

    #[test]
    fn test_field_name() {
        let field = FieldRef::field_name("created_at");
        assert_eq!(
            field.to_json(),
            json!(["field-literal", "created_at", "type/*"])
        );
    }

    #[test]
    fn test_field_name_typed() {
        let field = FieldRef::field_name_typed("created_at", FieldType::DateTime);
        assert_eq!(
            field.to_json(),
            json!(["field-literal", "created_at", "type/DateTime"])
        );
    }

    #[test]
    fn test_foreign_key() {
        let source = FieldRef::field_id(10);
        let target = FieldRef::field_id(20);
        let fk = FieldRef::foreign_key(source, target);

        assert_eq!(
            fk.to_json(),
            json!(["fk->", ["field-id", 10], ["field-id", 20]])
        );
    }

    #[test]
    fn test_expression() {
        let expr = FieldRef::expression("calculated_field");
        assert_eq!(expr.to_json(), json!(["expression", "calculated_field"]));
    }
}
