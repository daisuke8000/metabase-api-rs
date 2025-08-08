//! Unit tests for MBQL query builder

#[cfg(feature = "query-builder")]
#[cfg(test)]
mod tests {
    use metabase_api_rs::core::models::mbql::{
        Aggregation, FieldRef, Filter, MbqlQuery, OrderBy, OrderDirection,
    };
    use metabase_api_rs::core::models::MetabaseId;
    use serde_json::json;

    #[test]
    fn test_simple_mbql_query() {
        // Simple query: SELECT COUNT(*) FROM table_id = 1
        let query = MbqlQuery::builder()
            .source_table(MetabaseId(1))
            .aggregate(Aggregation::count())
            .build();

        assert_eq!(query.source_table(), Some(MetabaseId(1)));
        assert_eq!(query.aggregations().len(), 1);
    }

    #[test]
    fn test_mbql_query_with_filter() {
        // Query with filter: WHERE status = 'active'
        let query = MbqlQuery::builder()
            .source_table(MetabaseId(1))
            .filter(Filter::equals(FieldRef::field_id(10), json!("active")))
            .build();

        assert!(query.filter().is_some());
    }

    #[test]
    fn test_mbql_query_with_groupby() {
        // Query with GROUP BY
        let query = MbqlQuery::builder()
            .source_table(MetabaseId(1))
            .aggregate(Aggregation::sum(FieldRef::field_id(20)))
            .breakout(FieldRef::field_id(10))
            .build();

        assert_eq!(query.breakout().len(), 1);
        assert_eq!(query.aggregations().len(), 1);
    }

    #[test]
    fn test_mbql_query_serialization() {
        let query = MbqlQuery::builder()
            .source_table(MetabaseId(1))
            .aggregate(Aggregation::count())
            .build();

        let json = query.to_json().unwrap();

        assert_eq!(json["source-table"], 1);
        assert!(json["aggregation"].is_array());
    }

    #[test]
    fn test_complex_filter() {
        // Complex filter: (price > 100 AND category = 'electronics') OR featured = true
        let filter = Filter::or(vec![
            Filter::and(vec![
                Filter::greater_than(FieldRef::field_id(10), json!(100)),
                Filter::equals(FieldRef::field_id(11), json!("electronics")),
            ]),
            Filter::equals(FieldRef::field_id(12), json!(true)),
        ]);

        let query = MbqlQuery::builder()
            .source_table(MetabaseId(1))
            .filter(filter)
            .build();

        assert!(query.filter().is_some());
    }

    #[test]
    fn test_order_by() {
        let query = MbqlQuery::builder()
            .source_table(MetabaseId(1))
            .order_by(vec![
                OrderBy::new(FieldRef::field_id(10), OrderDirection::Asc),
                OrderBy::new(FieldRef::field_id(11), OrderDirection::Desc),
            ])
            .build();

        assert_eq!(query.order_by().len(), 2);
    }

    #[test]
    fn test_aggregation_functions() {
        // Test various aggregation functions
        let aggregations = vec![
            Aggregation::count(),
            Aggregation::sum(FieldRef::field_id(10)),
            Aggregation::avg(FieldRef::field_id(10)),
            Aggregation::min(FieldRef::field_id(10)),
            Aggregation::max(FieldRef::field_id(10)),
            Aggregation::distinct(FieldRef::field_id(10)),
        ];

        for agg in aggregations {
            let query = MbqlQuery::builder()
                .source_table(MetabaseId(1))
                .aggregate(agg)
                .build();

            assert_eq!(query.aggregations().len(), 1);
        }
    }

    #[test]
    fn test_field_references() {
        // Test different field reference types
        let field_id = FieldRef::field_id(10);
        let field_name = FieldRef::field_name("created_at");

        assert_eq!(field_id.to_json(), json!(["field-id", 10]));
        assert_eq!(
            field_name.to_json(),
            json!(["field-literal", "created_at", "type/*"])
        );
    }

    #[test]
    fn test_limit_and_offset() {
        let query = MbqlQuery::builder()
            .source_table(MetabaseId(1))
            .limit(100)
            .offset(50)
            .build();

        assert_eq!(query.limit(), Some(100));
        assert_eq!(query.offset(), Some(50));
    }

    #[test]
    fn test_fluent_builder_pattern() {
        // Test the fluent builder pattern
        let query = MbqlQuery::builder()
            .source_table(MetabaseId(1))
            .aggregate(Aggregation::sum(FieldRef::field_id(20)))
            .breakout(FieldRef::field_id(10))
            .filter(Filter::greater_than(FieldRef::field_id(20), json!(0)))
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

    #[test]
    fn test_empty_query() {
        // Test that we can create an empty query
        let query = MbqlQuery::builder().build();

        assert!(query.source_table().is_none());
        assert_eq!(query.aggregations().len(), 0);
        assert_eq!(query.breakout().len(), 0);
        assert!(query.filter().is_none());
    }

    #[test]
    fn test_multiple_aggregations() {
        let query = MbqlQuery::builder()
            .source_table(MetabaseId(1))
            .aggregate(Aggregation::count())
            .aggregate(Aggregation::sum(FieldRef::field_id(10)))
            .aggregate(Aggregation::avg(FieldRef::field_id(11)))
            .build();

        assert_eq!(query.aggregations().len(), 3);
    }

    #[test]
    fn test_multiple_breakouts() {
        let query = MbqlQuery::builder()
            .source_table(MetabaseId(1))
            .breakout(FieldRef::field_id(10))
            .breakout(FieldRef::field_id(11))
            .breakout(FieldRef::field_id(12))
            .build();

        assert_eq!(query.breakout().len(), 3);
    }

    #[test]
    fn test_filter_operations() {
        // Test all filter operations
        let filters = vec![
            Filter::equals(FieldRef::field_id(1), json!("value")),
            Filter::not_equals(FieldRef::field_id(1), json!("value")),
            Filter::less_than(FieldRef::field_id(1), json!(10)),
            Filter::less_than_or_equal(FieldRef::field_id(1), json!(10)),
            Filter::greater_than(FieldRef::field_id(1), json!(10)),
            Filter::greater_than_or_equal(FieldRef::field_id(1), json!(10)),
            Filter::between(FieldRef::field_id(1), json!(1), json!(10)),
            Filter::is_null(FieldRef::field_id(1)),
            Filter::not_null(FieldRef::field_id(1)),
            Filter::contains(FieldRef::field_id(1), json!("substring")),
            Filter::starts_with(FieldRef::field_id(1), json!("prefix")),
            Filter::ends_with(FieldRef::field_id(1), json!("suffix")),
        ];

        for filter in filters {
            let query = MbqlQuery::builder()
                .source_table(MetabaseId(1))
                .filter(filter)
                .build();

            assert!(query.filter().is_some());
        }
    }
}
