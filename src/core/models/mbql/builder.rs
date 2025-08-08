//! Builder pattern for MBQL queries

use super::{Aggregation, FieldRef, Filter, MbqlQuery, OrderBy};
use crate::core::models::MetabaseId;

/// Builder for creating MBQL queries
pub struct MbqlQueryBuilder {
    source_table: Option<MetabaseId>,
    aggregations: Vec<Aggregation>,
    breakout: Vec<FieldRef>,
    filter: Option<Filter>,
    order_by: Vec<OrderBy>,
    limit: Option<u32>,
    offset: Option<u32>,
    fields: Vec<FieldRef>,
}

impl MbqlQueryBuilder {
    /// Create a new query builder
    pub fn new() -> Self {
        Self {
            source_table: None,
            aggregations: Vec::new(),
            breakout: Vec::new(),
            filter: None,
            order_by: Vec::new(),
            limit: None,
            offset: None,
            fields: Vec::new(),
        }
    }

    /// Set the source table
    pub fn source_table(mut self, table_id: MetabaseId) -> Self {
        self.source_table = Some(table_id);
        self
    }

    /// Add an aggregation
    pub fn aggregate(mut self, aggregation: Aggregation) -> Self {
        self.aggregations.push(aggregation);
        self
    }

    /// Add multiple aggregations
    pub fn aggregations(mut self, aggregations: Vec<Aggregation>) -> Self {
        self.aggregations.extend(aggregations);
        self
    }

    /// Add a breakout field (GROUP BY)
    pub fn breakout(mut self, field: FieldRef) -> Self {
        self.breakout.push(field);
        self
    }

    /// Add multiple breakout fields
    pub fn breakouts(mut self, fields: Vec<FieldRef>) -> Self {
        self.breakout.extend(fields);
        self
    }

    /// Set the filter
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Add an order by clause
    pub fn order_by(mut self, order_by: Vec<OrderBy>) -> Self {
        self.order_by = order_by;
        self
    }

    /// Add a single order by clause
    pub fn order_by_one(mut self, order_by: OrderBy) -> Self {
        self.order_by.push(order_by);
        self
    }

    /// Set the limit
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the offset
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Add a field to include in the result
    pub fn field(mut self, field: FieldRef) -> Self {
        self.fields.push(field);
        self
    }

    /// Add multiple fields to include in the result
    pub fn fields(mut self, fields: Vec<FieldRef>) -> Self {
        self.fields.extend(fields);
        self
    }

    /// Build the query
    pub fn build(self) -> MbqlQuery {
        let mut query = MbqlQuery::default();
        query.source_table = self.source_table;
        query.aggregations = self.aggregations;
        query.breakout = self.breakout;
        query.filter = self.filter;
        query.order_by = self.order_by;
        query.limit = self.limit;
        query.offset = self.offset;
        query.fields = self.fields;
        query
    }
}

impl Default for MbqlQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_builder_basic() {
        let query = MbqlQueryBuilder::new().source_table(MetabaseId(1)).build();

        assert_eq!(query.source_table(), Some(MetabaseId(1)));
        assert!(query.aggregations().is_empty());
        assert!(query.breakout().is_empty());
        assert!(query.filter().is_none());
    }

    #[test]
    fn test_builder_with_aggregation() {
        let query = MbqlQueryBuilder::new()
            .source_table(MetabaseId(1))
            .aggregate(Aggregation::count())
            .aggregate(Aggregation::sum(FieldRef::field_id(10)))
            .build();

        assert_eq!(query.aggregations().len(), 2);
    }

    #[test]
    fn test_builder_with_filter() {
        let query = MbqlQueryBuilder::new()
            .source_table(MetabaseId(1))
            .filter(Filter::equals(FieldRef::field_id(10), json!("active")))
            .build();

        assert!(query.filter().is_some());
    }

    #[test]
    fn test_builder_with_breakout() {
        let query = MbqlQueryBuilder::new()
            .source_table(MetabaseId(1))
            .breakout(FieldRef::field_id(10))
            .breakout(FieldRef::field_id(11))
            .build();

        assert_eq!(query.breakout().len(), 2);
    }

    #[test]
    fn test_builder_with_order_by() {
        let query = MbqlQueryBuilder::new()
            .source_table(MetabaseId(1))
            .order_by_one(OrderBy::asc(FieldRef::field_id(10)))
            .order_by_one(OrderBy::desc(FieldRef::field_id(11)))
            .build();

        assert_eq!(query.order_by().len(), 2);
    }

    #[test]
    fn test_builder_with_limit_offset() {
        let query = MbqlQueryBuilder::new()
            .source_table(MetabaseId(1))
            .limit(100)
            .offset(50)
            .build();

        assert_eq!(query.limit(), Some(100));
        assert_eq!(query.offset(), Some(50));
    }

    #[test]
    fn test_builder_complex_query() {
        let query = MbqlQueryBuilder::new()
            .source_table(MetabaseId(1))
            .aggregate(Aggregation::sum(FieldRef::field_id(20)))
            .breakout(FieldRef::field_id(10))
            .filter(Filter::and(vec![
                Filter::greater_than(FieldRef::field_id(20), json!(0)),
                Filter::equals(FieldRef::field_id(11), json!("active")),
            ]))
            .order_by(vec![OrderBy::desc(FieldRef::field_id(20))])
            .limit(100)
            .build();

        assert_eq!(query.source_table(), Some(MetabaseId(1)));
        assert_eq!(query.aggregations().len(), 1);
        assert_eq!(query.breakout().len(), 1);
        assert!(query.filter().is_some());
        assert_eq!(query.order_by().len(), 1);
        assert_eq!(query.limit(), Some(100));
    }
}
