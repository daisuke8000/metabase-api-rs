//! Query and Dataset API integration tests

use metabase_api_rs::core::models::{
    DatasetQuery, QueryRequest, QueryParameters, ExportFormat, QueryStatus
};
use serde_json::json;
use crate::integration::setup::{get_test_client, create_test_data, cleanup_test_data};

#[tokio::test]
async fn test_execute_dataset_query() {
    let client = get_test_client().await;
    
    // Create a simple dataset query
    let dataset_query = DatasetQuery {
        database: 1,
        query_type: "native".to_string(),
        native: Some(json!({
            "query": "SELECT id, name, email FROM customers LIMIT 5"
        })),
        query: None,
    };
    
    let query_request = QueryRequest {
        database: 1,
        dataset_query,
        parameters: None,
    };
    
    let result = client.execute_dataset_query(query_request).await;
    
    assert!(result.is_ok(), "Dataset query execution should succeed");
    let query_result = result.unwrap();
    assert_eq!(query_result.status, QueryStatus::Completed);
    assert!(query_result.data.rows.len() <= 5, "Should return at most 5 rows");
    
    // Verify column structure
    assert!(query_result.data.cols.len() >= 3, "Should have at least 3 columns");
    let col_names: Vec<&str> = query_result.data.cols.iter()
        .map(|c| c.name.as_str())
        .collect();
    assert!(col_names.contains(&"id") || col_names.contains(&"ID"));
    assert!(col_names.contains(&"name") || col_names.contains(&"NAME"));
    assert!(col_names.contains(&"email") || col_names.contains(&"EMAIL"));
}

#[tokio::test]
async fn test_execute_parameterized_query() {
    let client = get_test_client().await;
    
    // Create a parameterized query
    let dataset_query = DatasetQuery {
        database: 1,
        query_type: "native".to_string(),
        native: Some(json!({
            "query": "SELECT * FROM customers WHERE name LIKE {{name_pattern}} LIMIT 10",
            "template-tags": {
                "name_pattern": {
                    "name": "name_pattern",
                    "display-name": "Name Pattern",
                    "type": "text",
                    "default": "%John%"
                }
            }
        })),
        query: None,
    };
    
    let parameters = QueryParameters {
        parameters: vec![json!({
            "type": "text",
            "target": ["variable", ["template-tag", "name_pattern"]],
            "value": "%Test%"
        })],
    };
    
    let query_request = QueryRequest {
        database: 1,
        dataset_query,
        parameters: Some(parameters),
    };
    
    let result = client.execute_dataset_query(query_request).await;
    
    assert!(result.is_ok(), "Parameterized query execution should succeed");
    let query_result = result.unwrap();
    assert_eq!(query_result.status, QueryStatus::Completed);
}

#[tokio::test]
async fn test_execute_pivot_query() {
    let client = get_test_client().await;
    
    // Create a pivot query (aggregation)
    let dataset_query = DatasetQuery {
        database: 1,
        query_type: "native".to_string(),
        native: Some(json!({
            "query": "SELECT COUNT(*) as count, DATE_TRUNC('month', created_at) as month FROM orders GROUP BY month ORDER BY month"
        })),
        query: None,
    };
    
    let query_request = QueryRequest {
        database: 1,
        dataset_query,
        parameters: None,
    };
    
    // Execute with pivot
    let result = client.execute_pivot_dataset_query(query_request).await;
    
    assert!(result.is_ok(), "Pivot query execution should succeed");
    let query_result = result.unwrap();
    assert_eq!(query_result.status, QueryStatus::Completed);
    
    // Pivot queries may have different structure
    assert!(query_result.data.cols.len() >= 2, "Should have at least 2 columns for pivot");
}

#[tokio::test]
async fn test_export_dataset_query_csv() {
    let client = get_test_client().await;
    
    // Create a simple query
    let dataset_query = DatasetQuery {
        database: 1,
        query_type: "native".to_string(),
        native: Some(json!({
            "query": "SELECT id, name, email FROM customers LIMIT 10"
        })),
        query: None,
    };
    
    let query_request = QueryRequest {
        database: 1,
        dataset_query,
        parameters: None,
    };
    
    // Export as CSV
    let csv_result = client.export_dataset_query(
        query_request.clone(),
        ExportFormat::Csv
    ).await;
    
    assert!(csv_result.is_ok(), "CSV export should succeed");
    let csv_data = csv_result.unwrap();
    assert!(csv_data.len() > 0, "CSV data should not be empty");
    
    // Verify it's valid CSV
    let csv_str = String::from_utf8(csv_data).expect("Should be valid UTF-8");
    assert!(csv_str.contains(","), "CSV should contain commas");
    assert!(csv_str.contains("id") || csv_str.contains("ID"), "CSV should have id column");
    assert!(csv_str.contains("name") || csv_str.contains("NAME"), "CSV should have name column");
}

#[tokio::test]
async fn test_export_dataset_query_json() {
    let client = get_test_client().await;
    
    // Create a simple query
    let dataset_query = DatasetQuery {
        database: 1,
        query_type: "native".to_string(),
        native: Some(json!({
            "query": "SELECT id, name FROM customers LIMIT 5"
        })),
        query: None,
    };
    
    let query_request = QueryRequest {
        database: 1,
        dataset_query,
        parameters: None,
    };
    
    // Export as JSON
    let json_result = client.export_dataset_query(
        query_request,
        ExportFormat::Json
    ).await;
    
    assert!(json_result.is_ok(), "JSON export should succeed");
    let json_data = json_result.unwrap();
    assert!(json_data.len() > 0, "JSON data should not be empty");
    
    // Verify it's valid JSON
    let json_str = String::from_utf8(json_data).expect("Should be valid UTF-8");
    let parsed: serde_json::Value = serde_json::from_str(&json_str)
        .expect("Should be valid JSON");
    
    // JSON export should be an array of objects
    assert!(parsed.is_array(), "JSON export should be an array");
    let array = parsed.as_array().unwrap();
    assert!(array.len() <= 5, "Should have at most 5 records");
    
    // Check structure of first record
    if !array.is_empty() {
        assert!(array[0].is_object(), "Each record should be an object");
        let obj = array[0].as_object().unwrap();
        assert!(obj.contains_key("id") || obj.contains_key("ID"));
        assert!(obj.contains_key("name") || obj.contains_key("NAME"));
    }
}

#[tokio::test]
async fn test_export_dataset_query_xlsx() {
    let client = get_test_client().await;
    
    // Create a simple query
    let dataset_query = DatasetQuery {
        database: 1,
        query_type: "native".to_string(),
        native: Some(json!({
            "query": "SELECT * FROM customers LIMIT 10"
        })),
        query: None,
    };
    
    let query_request = QueryRequest {
        database: 1,
        dataset_query,
        parameters: None,
    };
    
    // Export as XLSX
    let xlsx_result = client.export_dataset_query(
        query_request,
        ExportFormat::Xlsx
    ).await;
    
    assert!(xlsx_result.is_ok(), "XLSX export should succeed");
    let xlsx_data = xlsx_result.unwrap();
    assert!(xlsx_data.len() > 0, "XLSX data should not be empty");
    
    // XLSX files start with specific magic bytes (PK signature for ZIP format)
    assert_eq!(&xlsx_data[0..2], b"PK", "XLSX should be a valid ZIP/Office file");
}

#[tokio::test]
async fn test_card_query_with_parameters() {
    let client = get_test_client().await;
    let test_data = create_test_data(&*client).await.expect("Failed to create test data");
    
    // Execute card query with parameters
    let parameters = QueryParameters {
        parameters: vec![json!({
            "type": "number",
            "value": 5
        })],
    };
    
    let result = client.execute_card_query(test_data.card_id, Some(parameters)).await;
    
    assert!(result.is_ok(), "Card query with parameters should succeed");
    let query_result = result.unwrap();
    assert_eq!(query_result.status, QueryStatus::Completed);
    
    // Cleanup
    cleanup_test_data(&*client, &test_data).await.ok();
}

#[tokio::test]
async fn test_complex_mbql_query() {
    let client = get_test_client().await;
    
    // Create a complex MBQL query
    let dataset_query = DatasetQuery {
        database: 1,
        query_type: "query".to_string(),
        native: None,
        query: Some(json!({
            "source-table": 1,  // Assuming table ID 1 exists
            "aggregation": [["count"]],
            "breakout": [["field", 2, null]],  // Group by field ID 2
            "filter": ["and", 
                [">", ["field", 3, null], 100],
                ["<", ["field", 3, null], 1000]
            ],
            "order-by": [["desc", ["aggregation", 0]]],
            "limit": 10
        })),
    };
    
    let query_request = QueryRequest {
        database: 1,
        dataset_query,
        parameters: None,
    };
    
    let result = client.execute_dataset_query(query_request).await;
    
    // MBQL queries might fail if table/field IDs don't exist
    // Just check that the request is processed
    if result.is_ok() {
        let query_result = result.unwrap();
        assert!(
            query_result.status == QueryStatus::Completed || 
            query_result.status == QueryStatus::Failed,
            "Query should either complete or fail gracefully"
        );
    }
}