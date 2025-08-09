//! Parameter types for Cards and Dashboards

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Parameter definition for cards and dashboards
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Parameter {
    /// Unique identifier for the parameter
    pub id: String,

    /// Parameter type (e.g., "date/relative", "string/=", "number/>=")
    #[serde(rename = "type")]
    pub param_type: String,

    /// Display name for the parameter
    pub name: String,

    /// URL slug for the parameter
    pub slug: String,

    /// Default value for the parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,

    /// Whether the parameter is required
    #[serde(default)]
    pub required: bool,

    /// Options for the parameter (for select/dropdown types)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ParameterOption>>,

    /// Values for list parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_source_type: Option<String>,

    /// Configuration for values source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_source_config: Option<Value>,
}

/// Option for select/dropdown parameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterOption {
    /// Display text
    pub name: String,

    /// Actual value
    pub value: Value,
}

/// Parameter mapping for dashboard cards
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterMapping {
    /// Parameter ID being mapped
    pub parameter_id: String,

    /// Card ID this mapping applies to
    pub card_id: i64,

    /// Target for the mapping
    pub target: ParameterTarget,
}

/// Target for parameter mapping
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ParameterTarget {
    /// Variable target (e.g., for SQL parameters)
    Variable(VariableTarget),

    /// Dimension target (e.g., for field filters)
    Dimension(DimensionTarget),
}

/// Variable target for SQL parameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableTarget {
    /// Type is always "variable" for this variant
    #[serde(rename = "type")]
    pub target_type: String,

    /// Variable name
    pub id: String,
}

/// Dimension target for field filters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DimensionTarget {
    /// Type is always "dimension" for this variant
    #[serde(rename = "type")]
    pub target_type: String,

    /// Dimension specification [type, field_id/name, options]
    pub id: Vec<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parameter_serialization() {
        let param = Parameter {
            id: "date_param".to_string(),
            param_type: "date/relative".to_string(),
            name: "Date".to_string(),
            slug: "date".to_string(),
            default: Some(json!("past7days")),
            required: false,
            options: None,
            values_source_type: None,
            values_source_config: None,
        };

        let json = serde_json::to_value(&param).unwrap();
        assert_eq!(json["id"], "date_param");
        assert_eq!(json["type"], "date/relative");
        assert_eq!(json["name"], "Date");
        assert_eq!(json["slug"], "date");
        assert_eq!(json["default"], "past7days");
    }

    #[test]
    fn test_parameter_with_options() {
        let param = Parameter {
            id: "status_param".to_string(),
            param_type: "string/=".to_string(),
            name: "Status".to_string(),
            slug: "status".to_string(),
            default: Some(json!("active")),
            required: true,
            options: Some(vec![
                ParameterOption {
                    name: "Active".to_string(),
                    value: json!("active"),
                },
                ParameterOption {
                    name: "Inactive".to_string(),
                    value: json!("inactive"),
                },
            ]),
            values_source_type: None,
            values_source_config: None,
        };

        let json = serde_json::to_value(&param).unwrap();
        assert_eq!(json["required"], true);
        assert!(json["options"].is_array());
        assert_eq!(json["options"][0]["name"], "Active");
        assert_eq!(json["options"][0]["value"], "active");
    }

    #[test]
    fn test_variable_parameter_mapping() {
        let mapping = ParameterMapping {
            parameter_id: "date_param".to_string(),
            card_id: 123,
            target: ParameterTarget::Variable(VariableTarget {
                target_type: "variable".to_string(),
                id: "start_date".to_string(),
            }),
        };

        let json = serde_json::to_value(&mapping).unwrap();
        assert_eq!(json["parameter_id"], "date_param");
        assert_eq!(json["card_id"], 123);
        assert_eq!(json["target"]["type"], "variable");
        assert_eq!(json["target"]["id"], "start_date");
    }

    #[test]
    fn test_dimension_parameter_mapping() {
        let mapping = ParameterMapping {
            parameter_id: "category_param".to_string(),
            card_id: 456,
            target: ParameterTarget::Dimension(DimensionTarget {
                target_type: "dimension".to_string(),
                id: vec![json!("field"), json!(10), json!(null)],
            }),
        };

        let json = serde_json::to_value(&mapping).unwrap();
        assert_eq!(json["parameter_id"], "category_param");
        assert_eq!(json["card_id"], 456);
        assert_eq!(json["target"]["type"], "dimension");
        assert!(json["target"]["id"].is_array());
        assert_eq!(json["target"]["id"][0], "field");
        assert_eq!(json["target"]["id"][1], 10);
    }
}
