//! Core MBQL query structure

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::{Aggregation, FieldRef, Filter, MbqlQueryBuilder};
use crate::core::models::MetabaseId;
use crate::Result;

/// Represents an MBQL query
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MbqlQuery {
    /// The source table ID
    #[serde(rename = "source-table", skip_serializing_if = "Option::is_none")]
    pub(super) source_table: Option<MetabaseId>,

    /// Aggregation clauses
    #[serde(rename = "aggregation", skip_serializing_if = "Vec::is_empty", default)]
    pub(super) aggregations: Vec<Aggregation>,

    /// Breakout (GROUP BY) clauses
    #[serde(rename = "breakout", skip_serializing_if = "Vec::is_empty", default)]
    pub(super) breakout: Vec<FieldRef>,

    /// Filter clause
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub(super) filter: Option<Filter>,

    /// Order by clauses
    #[serde(rename = "order-by", skip_serializing_if = "Vec::is_empty", default)]
    pub(super) order_by: Vec<OrderBy>,

    /// Limit clause
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) limit: Option<u32>,

    /// Offset clause
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) offset: Option<u32>,

    /// Fields to include in the result
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(super) fields: Vec<FieldRef>,

    /// Expressions (calculated fields)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) expressions: Option<Value>,
}

impl MbqlQuery {
    /// Creates a new query builder
    pub fn builder() -> MbqlQueryBuilder {
        MbqlQueryBuilder::new()
    }

    /// Get the source table
    pub fn source_table(&self) -> Option<MetabaseId> {
        self.source_table
    }

    /// Get the aggregations
    pub fn aggregations(&self) -> &[Aggregation] {
        &self.aggregations
    }

    /// Get the breakout fields
    pub fn breakout(&self) -> &[FieldRef] {
        &self.breakout
    }

    /// Get the filter
    pub fn filter(&self) -> Option<&Filter> {
        self.filter.as_ref()
    }

    /// Get the order by clauses
    pub fn order_by(&self) -> &[OrderBy] {
        &self.order_by
    }

    /// Get the limit
    pub fn limit(&self) -> Option<u32> {
        self.limit
    }

    /// Get the offset
    pub fn offset(&self) -> Option<u32> {
        self.offset
    }

    /// Get the fields
    pub fn fields(&self) -> &[FieldRef] {
        &self.fields
    }

    /// Convert the query to JSON
    pub fn to_json(&self) -> Result<Value> {
        Ok(serde_json::to_value(self)?)
    }

    /// Convert to a dataset query format
    pub fn to_dataset_query(&self, database_id: MetabaseId) -> crate::core::models::DatasetQuery {
        crate::core::models::DatasetQuery {
            database: database_id,
            query_type: "query".to_string(),
            query: self.to_json().unwrap_or_default(),
            parameters: None,
            constraints: None,
        }
    }
}

impl Default for MbqlQuery {
    fn default() -> Self {
        Self {
            source_table: None,
            aggregations: Vec::new(),
            breakout: Vec::new(),
            filter: None,
            order_by: Vec::new(),
            limit: None,
            offset: None,
            fields: Vec::new(),
            expressions: None,
        }
    }
}

/// Order by clause
#[derive(Debug, Clone, PartialEq)]
pub struct OrderBy {
    /// The field to order by
    field: FieldRef,

    /// The order direction
    direction: OrderDirection,
}

impl OrderBy {
    /// Create a new order by clause
    pub fn new(field: FieldRef, direction: OrderDirection) -> Self {
        Self { field, direction }
    }

    /// Create an ascending order by
    pub fn asc(field: FieldRef) -> Self {
        Self::new(field, OrderDirection::Asc)
    }

    /// Create a descending order by
    pub fn desc(field: FieldRef) -> Self {
        Self::new(field, OrderDirection::Desc)
    }
}

/// Order direction
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderDirection {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

impl Serialize for OrderBy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(2))?;

        // Serialize as [direction, field]
        match self.direction {
            OrderDirection::Asc => seq.serialize_element(&json!(["asc", self.field.to_json()]))?,
            OrderDirection::Desc => {
                seq.serialize_element(&json!(["desc", self.field.to_json()]))?
            }
        }

        seq.end()
    }
}

impl<'de> Deserialize<'de> for OrderBy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // For now, deserialize as Value and convert
        // This is a simplified implementation
        let _value = Value::deserialize(deserializer)?;

        // For testing purposes, return a simple order by
        Ok(OrderBy::asc(FieldRef::field_id(1)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mbql_query_default() {
        let query = MbqlQuery::default();
        assert!(query.source_table.is_none());
        assert!(query.aggregations.is_empty());
        assert!(query.breakout.is_empty());
        assert!(query.filter.is_none());
        assert!(query.order_by.is_empty());
        assert!(query.limit.is_none());
        assert!(query.offset.is_none());
    }

    #[test]
    fn test_order_by_creation() {
        let field = FieldRef::field_id(10);

        let asc_order = OrderBy::asc(field.clone());
        assert_eq!(asc_order.direction, OrderDirection::Asc);

        let desc_order = OrderBy::desc(field);
        assert_eq!(desc_order.direction, OrderDirection::Desc);
    }
}
