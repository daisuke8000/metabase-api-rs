//! Transport layer traits for abstraction

use crate::core::error::Result;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;

/// HTTP communication abstraction trait
///
/// This trait provides a generic interface for HTTP operations,
/// allowing for different implementations (e.g., production HTTP client, mock for testing)
#[async_trait]
pub trait HttpProvider: Send + Sync {
    /// Execute a GET request
    async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned + Send;

    /// Execute a POST request with JSON body
    async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned + Send,
        B: Serialize + Send + Sync + ?Sized;

    /// Execute a PUT request with JSON body
    async fn put<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned + Send,
        B: Serialize + Send + Sync + ?Sized;

    /// Execute a DELETE request
    async fn delete(&self, path: &str) -> Result<()>;

    /// Execute a POST request that returns binary data
    async fn post_binary<B>(&self, path: &str, body: &B) -> Result<Vec<u8>>
    where
        B: Serialize + Send + Sync + ?Sized;

    /// Set the session token for authenticated requests
    fn set_session_token(&mut self, token: Option<String>);

    /// Set the request timeout
    fn set_timeout(&mut self, timeout: Duration);

    /// Get response metadata for the last request (optional)
    fn last_response_metadata(&self) -> Option<ResponseMetadata> {
        None
    }
}

/// HTTP response metadata
#[derive(Debug, Clone)]
pub struct ResponseMetadata {
    /// HTTP status code
    pub status: u16,
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Request duration
    pub elapsed: Duration,
    /// Response size in bytes
    pub size: Option<usize>,
}

impl ResponseMetadata {
    /// Create a new ResponseMetadata
    pub fn new(status: u16) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            elapsed: Duration::from_secs(0),
            size: None,
        }
    }

    /// Set the elapsed time
    pub fn with_elapsed(mut self, elapsed: Duration) -> Self {
        self.elapsed = elapsed;
        self
    }

    /// Add a header
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Set the response size
    pub fn with_size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }
}

/// A no-op HTTP provider for testing
#[derive(Debug, Clone)]
pub struct NoOpHttpProvider;

#[async_trait]
impl HttpProvider for NoOpHttpProvider {
    async fn get<T>(&self, _path: &str) -> Result<T>
    where
        T: DeserializeOwned + Send,
    {
        Err(crate::core::error::Error::Network(
            "NoOp provider".to_string(),
        ))
    }

    async fn post<T, B>(&self, _path: &str, _body: &B) -> Result<T>
    where
        T: DeserializeOwned + Send,
        B: Serialize + Send + Sync + ?Sized,
    {
        Err(crate::core::error::Error::Network(
            "NoOp provider".to_string(),
        ))
    }

    async fn put<T, B>(&self, _path: &str, _body: &B) -> Result<T>
    where
        T: DeserializeOwned + Send,
        B: Serialize + Send + Sync + ?Sized,
    {
        Err(crate::core::error::Error::Network(
            "NoOp provider".to_string(),
        ))
    }

    async fn delete(&self, _path: &str) -> Result<()> {
        Err(crate::core::error::Error::Network(
            "NoOp provider".to_string(),
        ))
    }

    async fn post_binary<B>(&self, _path: &str, _body: &B) -> Result<Vec<u8>>
    where
        B: Serialize + Send + Sync + ?Sized,
    {
        Err(crate::core::error::Error::Network(
            "NoOp provider".to_string(),
        ))
    }

    fn set_session_token(&mut self, _token: Option<String>) {}

    fn set_timeout(&mut self, _timeout: Duration) {}
}
