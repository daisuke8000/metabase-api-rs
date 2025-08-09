//! Object-safe HTTP Provider trait
//!
//! This module provides an object-safe version of HttpProvider trait
//! that can be used with dynamic dispatch.

use crate::core::error::Result;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

/// Object-safe HTTP provider trait
#[async_trait]
pub trait HttpProviderSafe: Send + Sync {
    /// Execute a GET request returning JSON
    async fn get_json(&self, path: &str) -> Result<Value>;

    /// Execute a POST request with JSON body
    async fn post_json(&self, path: &str, body: Value) -> Result<Value>;

    /// Execute a PUT request with JSON body
    async fn put_json(&self, path: &str, body: Value) -> Result<Value>;

    /// Execute a DELETE request
    async fn delete_json(&self, path: &str) -> Result<Value>;
}

/// Extension trait for typed operations
#[async_trait]
pub trait HttpProviderExt: HttpProviderSafe {
    /// Get with type conversion
    async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned + Send,
    {
        let json = self.get_json(path).await?;
        serde_json::from_value(json)
            .map_err(|e| crate::core::error::Error::Serialization(e.to_string()))
    }

    /// Post with type conversion
    async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned + Send,
        B: Serialize + Send + Sync,
    {
        let body_json = serde_json::to_value(body)
            .map_err(|e| crate::core::error::Error::Serialization(e.to_string()))?;
        let json = self.post_json(path, body_json).await?;
        serde_json::from_value(json)
            .map_err(|e| crate::core::error::Error::Serialization(e.to_string()))
    }

    /// Put with type conversion
    async fn put<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned + Send,
        B: Serialize + Send + Sync,
    {
        let body_json = serde_json::to_value(body)
            .map_err(|e| crate::core::error::Error::Serialization(e.to_string()))?;
        let json = self.put_json(path, body_json).await?;
        serde_json::from_value(json)
            .map_err(|e| crate::core::error::Error::Serialization(e.to_string()))
    }

    /// Delete with type conversion
    async fn delete<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned + Send,
    {
        let json = self.delete_json(path).await?;
        serde_json::from_value(json)
            .map_err(|e| crate::core::error::Error::Serialization(e.to_string()))
    }
}

/// Automatically implement HttpProviderExt for all HttpProviderSafe types
impl<T: HttpProviderSafe + ?Sized> HttpProviderExt for T {}

/// Adapter for existing HttpClient
use crate::transport::HttpClient;

pub struct HttpClientAdapter {
    client: HttpClient,
}

impl HttpClientAdapter {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl HttpProviderSafe for HttpClientAdapter {
    async fn get_json(&self, path: &str) -> Result<Value> {
        self.client.get(path).await
    }

    async fn post_json(&self, path: &str, body: Value) -> Result<Value> {
        self.client.post(path, &body).await
    }

    async fn put_json(&self, path: &str, body: Value) -> Result<Value> {
        self.client.put(path, &body).await
    }

    async fn delete_json(&self, path: &str) -> Result<Value> {
        self.client.delete(path).await?;
        Ok(serde_json::json!({}))
    }
}
