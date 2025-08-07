use metabase_api_rs::transport::http::{HttpClient, HttpClientBuilder};
use metabase_api_rs::Result;
use mockito::{mock, Matcher};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestResponse {
    id: i64,
    name: String,
}

#[tokio::test]
async fn test_http_client_creation() {
    let client = HttpClient::new("https://metabase.example.com")
        .expect("Failed to create HTTP client");
    
    assert_eq!(client.base_url(), "https://metabase.example.com");
}

#[tokio::test]
async fn test_http_client_with_builder() {
    let client = HttpClientBuilder::new("https://metabase.example.com")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client");
    
    assert_eq!(client.base_url(), "https://metabase.example.com");
}

#[tokio::test]
async fn test_get_request() {
    let base_url = mockito::server_url();
    let _m = mock("GET", "/api/test")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id": 123, "name": "Test Item"}"#)
        .create();

    let client = HttpClient::new(&base_url).unwrap();
    let response: TestResponse = client.get("/api/test").await.unwrap();
    
    assert_eq!(response.id, 123);
    assert_eq!(response.name, "Test Item");
}

#[tokio::test]
async fn test_post_request() {
    let base_url = mockito::server_url();
    let request_body = TestResponse {
        id: 456,
        name: "New Item".to_string(),
    };
    
    let _m = mock("POST", "/api/test")
        .match_header("content-type", "application/json")
        .match_body(Matcher::Json(serde_json::json!({
            "id": 456,
            "name": "New Item"
        })))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id": 456, "name": "New Item"}"#)
        .create();

    let client = HttpClient::new(&base_url).unwrap();
    let response: TestResponse = client.post("/api/test", &request_body).await.unwrap();
    
    assert_eq!(response.id, 456);
    assert_eq!(response.name, "New Item");
}

#[tokio::test]
async fn test_put_request() {
    let base_url = mockito::server_url();
    let request_body = TestResponse {
        id: 789,
        name: "Updated Item".to_string(),
    };
    
    let _m = mock("PUT", "/api/test/789")
        .match_header("content-type", "application/json")
        .match_body(Matcher::Json(serde_json::json!({
            "id": 789,
            "name": "Updated Item"
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id": 789, "name": "Updated Item"}"#)
        .create();

    let client = HttpClient::new(&base_url).unwrap();
    let response: TestResponse = client.put("/api/test/789", &request_body).await.unwrap();
    
    assert_eq!(response.id, 789);
    assert_eq!(response.name, "Updated Item");
}

#[tokio::test]
async fn test_delete_request() {
    let base_url = mockito::server_url();
    let _m = mock("DELETE", "/api/test/999")
        .with_status(204)
        .create();

    let client = HttpClient::new(&base_url).unwrap();
    client.delete("/api/test/999").await.unwrap();
}

#[tokio::test]
async fn test_error_handling() {
    let base_url = mockito::server_url();
    let _m = mock("GET", "/api/error")
        .with_status(500)
        .with_body("Internal Server Error")
        .create();

    let client = HttpClient::new(&base_url).unwrap();
    let result: Result<TestResponse> = client.get("/api/error").await;
    
    assert!(result.is_err());
    // Error should indicate HTTP error
}

#[tokio::test]
async fn test_with_custom_headers() {
    let base_url = mockito::server_url();
    let _m = mock("GET", "/api/test")
        .match_header("x-custom-header", "custom-value")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id": 1, "name": "Test"}"#)
        .create();

    let client = HttpClientBuilder::new(&base_url)
        .header("x-custom-header", "custom-value")
        .build()
        .unwrap();
        
    let response: TestResponse = client.get("/api/test").await.unwrap();
    assert_eq!(response.id, 1);
}