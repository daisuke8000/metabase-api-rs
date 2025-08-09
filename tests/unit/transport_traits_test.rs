//! Tests for transport layer traits

use metabase_api_rs::transport::traits::{HttpProvider, ResponseMetadata};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestResponse {
    id: i64,
    name: String,
}

// Mock implementation for testing
struct MockHttpProvider {
    responses: HashMap<String, String>,
    should_fail: bool,
}

impl MockHttpProvider {
    fn new() -> Self {
        Self {
            responses: HashMap::new(),
            should_fail: false,
        }
    }
    
    fn with_response(mut self, path: &str, response: &str) -> Self {
        self.responses.insert(path.to_string(), response.to_string());
        self
    }
    
    fn with_failure(mut self) -> Self {
        self.should_fail = true;
        self
    }
}

#[async_trait]
impl HttpProvider for MockHttpProvider {
    async fn get<T>(&self, path: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned + Send,
    {
        if self.should_fail {
            return Err("Mock failure".into());
        }
        
        let response = self.responses.get(path)
            .ok_or("Not found")?;
        
        serde_json::from_str(response).map_err(Into::into)
    }
    
    async fn post<T, B>(&self, path: &str, _body: &B) -> Result<T, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned + Send,
        B: Serialize + Send + Sync + ?Sized,
    {
        self.get(path).await
    }
    
    async fn put<T, B>(&self, path: &str, _body: &B) -> Result<T, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned + Send,
        B: Serialize + Send + Sync + ?Sized,
    {
        self.get(path).await
    }
    
    async fn delete(&self, _path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.should_fail {
            return Err("Mock failure".into());
        }
        Ok(())
    }
    
    async fn post_binary<B>(&self, _path: &str, _body: &B) -> Result<Vec<u8>, Box<dyn std::error::Error>>
    where
        B: Serialize + Send + Sync + ?Sized,
    {
        if self.should_fail {
            return Err("Mock failure".into());
        }
        Ok(vec![1, 2, 3, 4])
    }
    
    fn set_session_token(&mut self, _token: Option<String>) {}
    
    fn set_timeout(&mut self, _timeout: Duration) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_http_provider_get() {
        let provider = MockHttpProvider::new()
            .with_response("/api/test", r#"{"id": 123, "name": "Test"}"#);
        
        let result: TestResponse = provider.get("/api/test").await.unwrap();
        assert_eq!(result.id, 123);
        assert_eq!(result.name, "Test");
    }
    
    #[tokio::test]
    async fn test_http_provider_error_handling() {
        let provider = MockHttpProvider::new()
            .with_failure();
        
        let result: Result<TestResponse, _> = provider.get("/api/test").await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_http_provider_post() {
        let provider = MockHttpProvider::new()
            .with_response("/api/create", r#"{"id": 456, "name": "Created"}"#);
        
        let body = TestResponse { id: 0, name: "New".to_string() };
        let result: TestResponse = provider.post("/api/create", &body).await.unwrap();
        assert_eq!(result.id, 456);
        assert_eq!(result.name, "Created");
    }
    
    #[tokio::test]
    async fn test_http_provider_delete() {
        let provider = MockHttpProvider::new();
        let result = provider.delete("/api/test/123").await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_http_provider_post_binary() {
        let provider = MockHttpProvider::new();
        let body = TestResponse { id: 1, name: "Binary".to_string() };
        let result = provider.post_binary("/api/export", &body).await.unwrap();
        assert_eq!(result, vec![1, 2, 3, 4]);
    }
}