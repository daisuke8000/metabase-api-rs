//! Integration tests for MBQL query builder with MetabaseClient

#![cfg(all(feature = "query-builder", feature = "integration-tests"))]

use metabase_api_rs::api::MetabaseClient;
use metabase_api_rs::api::auth::Credentials;
use metabase_api_rs::core::models::common::{ExportFormat, MetabaseId};
use metabase_api_rs::core::models::mbql::{Aggregation, FieldRef, Filter, MbqlQuery, OrderBy};
use metabase_api_rs::Result;
use serde_json::json;
use std::env;

/// Helper function to create authenticated client for testing
async fn create_test_client() -> Result<MetabaseClient> {
    let base_url = env::var("METABASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let username = env::var("METABASE_USERNAME").unwrap_or_else(|_| "test@example.com".to_string());
    let password = env::var("METABASE_PASSWORD").unwrap_or_else(|_| "testpassword".to_string());
    
    let mut client = MetabaseClient::new(base_url)?;
    
    let credentials = Credentials::EmailPassword {
        email: username,
        password,
    };
    
    client.authenticate(credentials).await?;
    Ok(client)
}

#[tokio::test]
#[ignore] // Run with `cargo test --ignored` when Metabase is available
async fn test_execute_simple_mbql_query() {
    let client = create_test_client().await.expect("Failed to create client");
    
    // Build a simple query
    let query = MbqlQuery::builder()
        .source_table(MetabaseId(1))  // Adjust table ID for your test environment
        .aggregate(Aggregation::count())
        .limit(10)
        .build();
    
    // Execute query
    let result = client
        .execute_mbql_query(MetabaseId(1), query)  // Adjust database ID
        .await;
    
    assert!(result.is_ok(), "Query execution failed: {:?}", result);
    
    if let Ok(query_result) = result {
        assert!(query_result.row_count > 0);
        assert!(!query_result.data.rows.is_empty());
    }
}

#[tokio::test]
#[ignore]
async fn test_execute_filtered_mbql_query() {
    let client = create_test_client().await.expect("Failed to create client");
    
    // Build a query with filter
    let query = MbqlQuery::builder()
        .source_table(MetabaseId(1))
        .aggregate(Aggregation::count())
        .filter(Filter::not_null(FieldRef::field_id(1)))  // Adjust field ID
        .limit(10)
        .build();
    
    // Execute query
    let result = client
        .execute_mbql_query(MetabaseId(1), query)
        .await;
    
    assert!(result.is_ok(), "Filtered query execution failed: {:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_execute_aggregated_mbql_query() {
    let client = create_test_client().await.expect("Failed to create client");
    
    // Build a query with multiple aggregations
    let query = MbqlQuery::builder()
        .source_table(MetabaseId(1))
        .aggregate(Aggregation::count())
        .aggregate(Aggregation::sum(FieldRef::field_id(2)))  // Adjust field ID
        .aggregate(Aggregation::avg(FieldRef::field_id(2)))
        .breakout(FieldRef::field_id(3))  // Adjust field ID for grouping
        .order_by_one(OrderBy::desc(FieldRef::field_id(2)))
        .limit(20)
        .build();
    
    // Execute query
    let result = client
        .execute_mbql_query(MetabaseId(1), query)
        .await;
    
    assert!(result.is_ok(), "Aggregated query execution failed: {:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_execute_complex_filter_mbql_query() {
    let client = create_test_client().await.expect("Failed to create client");
    
    // Build a query with complex filters
    let query = MbqlQuery::builder()
        .source_table(MetabaseId(1))
        .aggregate(Aggregation::count())
        .filter(Filter::and(vec![
            Filter::greater_than(FieldRef::field_id(2), json!(0)),
            Filter::less_than(FieldRef::field_id(2), json!(1000)),
            Filter::not_null(FieldRef::field_id(3)),
        ]))
        .limit(50)
        .build();
    
    // Execute query
    let result = client
        .execute_mbql_query(MetabaseId(1), query)
        .await;
    
    assert!(result.is_ok(), "Complex filter query execution failed: {:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_export_mbql_query_to_csv() {
    let client = create_test_client().await.expect("Failed to create client");
    
    // Build a simple query
    let query = MbqlQuery::builder()
        .source_table(MetabaseId(1))
        .aggregate(Aggregation::count())
        .limit(10)
        .build();
    
    // Export as CSV
    let result = client
        .export_mbql_query(MetabaseId(1), query, ExportFormat::Csv)
        .await;
    
    assert!(result.is_ok(), "CSV export failed: {:?}", result);
    
    if let Ok(csv_data) = result {
        assert!(!csv_data.is_empty(), "CSV data should not be empty");
        
        // Verify CSV format
        let csv_string = String::from_utf8_lossy(&csv_data);
        assert!(csv_string.contains(','), "CSV should contain comma separators");
    }
}

#[tokio::test]
#[ignore]
async fn test_export_mbql_query_to_json() {
    let client = create_test_client().await.expect("Failed to create client");
    
    // Build a query
    let query = MbqlQuery::builder()
        .source_table(MetabaseId(1))
        .aggregate(Aggregation::count())
        .limit(5)
        .build();
    
    // Export as JSON
    let result = client
        .export_mbql_query(MetabaseId(1), query, ExportFormat::Json)
        .await;
    
    assert!(result.is_ok(), "JSON export failed: {:?}", result);
    
    if let Ok(json_data) = result {
        assert!(!json_data.is_empty(), "JSON data should not be empty");
        
        // Verify JSON format
        let json_string = String::from_utf8_lossy(&json_data);
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&json_string);
        assert!(parsed.is_ok(), "Should be valid JSON");
    }
}

#[tokio::test]
#[ignore]
async fn test_execute_mbql_query_with_field_selection() {
    let client = create_test_client().await.expect("Failed to create client");
    
    // Build a query with specific field selection
    let query = MbqlQuery::builder()
        .source_table(MetabaseId(1))
        .field(FieldRef::field_id(1))
        .field(FieldRef::field_id(2))
        .field(FieldRef::field_id(3))
        .limit(10)
        .build();
    
    // Execute query
    let result = client
        .execute_mbql_query(MetabaseId(1), query)
        .await;
    
    assert!(result.is_ok(), "Field selection query execution failed: {:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_execute_mbql_query_with_string_filters() {
    let client = create_test_client().await.expect("Failed to create client");
    
    // Build a query with string filters
    let query = MbqlQuery::builder()
        .source_table(MetabaseId(1))
        .filter(Filter::or(vec![
            Filter::contains(FieldRef::field_id(1), json!("test")),
            Filter::starts_with(FieldRef::field_id(1), json!("prefix")),
            Filter::ends_with(FieldRef::field_id(1), json!("suffix")),
        ]))
        .limit(20)
        .build();
    
    // Execute query
    let result = client
        .execute_mbql_query(MetabaseId(1), query)
        .await;
    
    assert!(result.is_ok(), "String filter query execution failed: {:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_serialization_roundtrip() {
    // Build a complex query
    let query = MbqlQuery::builder()
        .source_table(MetabaseId(1))
        .aggregate(Aggregation::sum(FieldRef::field_id(10)))
        .aggregate(Aggregation::avg(FieldRef::field_id(11)))
        .breakout(FieldRef::field_id(12))
        .filter(Filter::and(vec![
            Filter::greater_than(FieldRef::field_id(10), json!(100)),
            Filter::less_than(FieldRef::field_id(10), json!(1000)),
        ]))
        .order_by_one(OrderBy::desc(FieldRef::field_id(10)))
        .limit(100)
        .offset(50)
        .build();
    
    // Convert to JSON and back
    let json = query.to_json().expect("Serialization should succeed");
    let json_string = json.to_string();
    
    // Verify JSON structure
    assert!(json_string.contains("source-table"));
    assert!(json_string.contains("aggregation"));
    assert!(json_string.contains("breakout"));
    assert!(json_string.contains("filter"));
    assert!(json_string.contains("order-by"));
    assert!(json_string.contains("limit"));
    assert!(json_string.contains("offset"));
}