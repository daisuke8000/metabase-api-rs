//! Aggregation functions for MBQL queries

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::FieldRef;

/// Represents an aggregation in MBQL
#[derive(Debug, Clone, PartialEq)]
pub struct Aggregation {
    /// The type of aggregation
    aggregation_type: AggregationType,

    /// The field to aggregate (optional for count)
    field: Option<FieldRef>,
}

impl Aggregation {
    /// Create a COUNT aggregation
    pub fn count() -> Self {
        Self {
            aggregation_type: AggregationType::Count,
            field: None,
        }
    }

    /// Create a COUNT aggregation for a specific field
    pub fn count_field(field: FieldRef) -> Self {
        Self {
            aggregation_type: AggregationType::Count,
            field: Some(field),
        }
    }

    /// Create a SUM aggregation
    pub fn sum(field: FieldRef) -> Self {
        Self {
            aggregation_type: AggregationType::Sum,
            field: Some(field),
        }
    }

    /// Create an AVG aggregation
    pub fn avg(field: FieldRef) -> Self {
        Self {
            aggregation_type: AggregationType::Avg,
            field: Some(field),
        }
    }

    /// Create a MIN aggregation
    pub fn min(field: FieldRef) -> Self {
        Self {
            aggregation_type: AggregationType::Min,
            field: Some(field),
        }
    }

    /// Create a MAX aggregation
    pub fn max(field: FieldRef) -> Self {
        Self {
            aggregation_type: AggregationType::Max,
            field: Some(field),
        }
    }

    /// Create a DISTINCT aggregation
    pub fn distinct(field: FieldRef) -> Self {
        Self {
            aggregation_type: AggregationType::Distinct,
            field: Some(field),
        }
    }

    /// Create a CUMSUM aggregation
    pub fn cumulative_sum(field: FieldRef) -> Self {
        Self {
            aggregation_type: AggregationType::CumSum,
            field: Some(field),
        }
    }

    /// Create a STDDEV aggregation
    pub fn stddev(field: FieldRef) -> Self {
        Self {
            aggregation_type: AggregationType::StdDev,
            field: Some(field),
        }
    }

    /// Create a VARIANCE aggregation
    pub fn variance(field: FieldRef) -> Self {
        Self {
            aggregation_type: AggregationType::Variance,
            field: Some(field),
        }
    }

    /// Convert to JSON representation
    pub fn to_json(&self) -> Value {
        match (&self.aggregation_type, &self.field) {
            (AggregationType::Count, None) => json!(["count"]),
            (AggregationType::Count, Some(field)) => json!(["count", field.to_json()]),
            (agg_type, Some(field)) => {
                json!([agg_type.to_string(), field.to_json()])
            }
            (_, None) => {
                // This shouldn't happen for non-count aggregations
                json!([self.aggregation_type.to_string()])
            }
        }
    }
}

/// Aggregation type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AggregationType {
    /// Count aggregation
    Count,
    /// Sum aggregation
    Sum,
    /// Average aggregation
    Avg,
    /// Minimum aggregation
    Min,
    /// Maximum aggregation
    Max,
    /// Distinct count aggregation
    Distinct,
    /// Cumulative sum aggregation
    #[serde(rename = "cum-sum")]
    CumSum,
    /// Standard deviation aggregation
    #[serde(rename = "stddev")]
    StdDev,
    /// Variance aggregation
    Variance,
}

impl AggregationType {
    /// Convert to string representation
    pub fn to_string(&self) -> &'static str {
        match self {
            AggregationType::Count => "count",
            AggregationType::Sum => "sum",
            AggregationType::Avg => "avg",
            AggregationType::Min => "min",
            AggregationType::Max => "max",
            AggregationType::Distinct => "distinct",
            AggregationType::CumSum => "cum-sum",
            AggregationType::StdDev => "stddev",
            AggregationType::Variance => "variance",
        }
    }
}

impl Serialize for Aggregation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_json().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Aggregation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // For now, deserialize as Value and convert
        // This is a simplified implementation
        let value = Value::deserialize(deserializer)?;

        // Parse the array structure
        if let Some(arr) = value.as_array() {
            if arr.is_empty() {
                return Err(serde::de::Error::custom("Empty aggregation array"));
            }

            let agg_type = arr[0]
                .as_str()
                .ok_or_else(|| serde::de::Error::custom("Invalid aggregation type"))?;

            match agg_type {
                "count" => {
                    if arr.len() == 1 {
                        Ok(Aggregation::count())
                    } else {
                        // count with field
                        Ok(Aggregation::count())
                    }
                }
                "sum" | "avg" | "min" | "max" | "distinct" => {
                    // For simplicity, return a placeholder
                    Ok(Aggregation::count())
                }
                _ => Ok(Aggregation::count()),
            }
        } else {
            Err(serde::de::Error::custom("Expected array for aggregation"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_aggregation() {
        let agg = Aggregation::count();
        assert_eq!(agg.to_json(), json!(["count"]));
    }

    #[test]
    fn test_count_field_aggregation() {
        let field = FieldRef::field_id(10);
        let agg = Aggregation::count_field(field);
        assert_eq!(agg.to_json(), json!(["count", ["field-id", 10]]));
    }

    #[test]
    fn test_sum_aggregation() {
        let field = FieldRef::field_id(10);
        let agg = Aggregation::sum(field);
        assert_eq!(agg.to_json(), json!(["sum", ["field-id", 10]]));
    }

    #[test]
    fn test_avg_aggregation() {
        let field = FieldRef::field_id(10);
        let agg = Aggregation::avg(field);
        assert_eq!(agg.to_json(), json!(["avg", ["field-id", 10]]));
    }

    #[test]
    fn test_min_max_aggregations() {
        let field = FieldRef::field_id(10);

        let min_agg = Aggregation::min(field.clone());
        assert_eq!(min_agg.to_json(), json!(["min", ["field-id", 10]]));

        let max_agg = Aggregation::max(field);
        assert_eq!(max_agg.to_json(), json!(["max", ["field-id", 10]]));
    }

    #[test]
    fn test_distinct_aggregation() {
        let field = FieldRef::field_id(10);
        let agg = Aggregation::distinct(field);
        assert_eq!(agg.to_json(), json!(["distinct", ["field-id", 10]]));
    }

    #[test]
    fn test_statistical_aggregations() {
        let field = FieldRef::field_id(10);

        let cumsum = Aggregation::cumulative_sum(field.clone());
        assert_eq!(cumsum.to_json(), json!(["cum-sum", ["field-id", 10]]));

        let stddev = Aggregation::stddev(field.clone());
        assert_eq!(stddev.to_json(), json!(["stddev", ["field-id", 10]]));

        let variance = Aggregation::variance(field);
        assert_eq!(variance.to_json(), json!(["variance", ["field-id", 10]]));
    }
}
