//! HttpProvider trait implementation for HttpClient

use super::http::HttpClient;
use super::traits::{HttpProvider, ResponseMetadata};
use crate::core::error::Result;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Extended HttpClient with HttpProvider implementation
pub struct HttpClientWithProvider {
    pub(crate) client: HttpClient,
    pub(crate) session_token: Option<String>,
    pub(crate) last_metadata: Arc<Mutex<Option<ResponseMetadata>>>,
}

impl HttpClientWithProvider {
    /// Create a new HttpClientWithProvider
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        Ok(Self {
            client: HttpClient::new(base_url)?,
            session_token: None,
            last_metadata: Arc::new(Mutex::new(None)),
        })
    }

    /// Build request headers including session token if available
    fn build_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();

        if let Some(ref token) = self.session_token {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(token) {
                headers.insert("X-Metabase-Session", value);
            }
        }

        headers
    }

    /// Store response metadata
    fn store_metadata(&self, status: u16, elapsed: Duration) {
        let metadata = ResponseMetadata::new(status).with_elapsed(elapsed);

        if let Ok(mut guard) = self.last_metadata.lock() {
            *guard = Some(metadata);
        }
    }
}

#[async_trait]
impl HttpProvider for HttpClientWithProvider {
    async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned + Send,
    {
        let start = Instant::now();
        let headers = self.build_headers();

        // Use the existing HttpClient get method with headers
        let response = self.client.get_with_headers(path, headers).await?;

        let elapsed = start.elapsed();
        // For now, we'll use a placeholder status since we need to modify HttpClient
        self.store_metadata(200, elapsed);

        Ok(response)
    }

    async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned + Send,
        B: Serialize + Send + Sync + ?Sized,
    {
        let start = Instant::now();
        let headers = self.build_headers();

        let response = self.client.post_with_headers(path, body, headers).await?;

        let elapsed = start.elapsed();
        self.store_metadata(200, elapsed);

        Ok(response)
    }

    async fn put<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned + Send,
        B: Serialize + Send + Sync + ?Sized,
    {
        let start = Instant::now();
        let headers = self.build_headers();

        let response = self.client.put_with_headers(path, body, headers).await?;

        let elapsed = start.elapsed();
        self.store_metadata(200, elapsed);

        Ok(response)
    }

    async fn delete(&self, path: &str) -> Result<()> {
        let start = Instant::now();
        let headers = self.build_headers();

        self.client.delete_with_headers(path, headers).await?;

        let elapsed = start.elapsed();
        self.store_metadata(200, elapsed);

        Ok(())
    }

    async fn post_binary<B>(&self, path: &str, body: &B) -> Result<Vec<u8>>
    where
        B: Serialize + Send + Sync + ?Sized,
    {
        let start = Instant::now();
        let headers = self.build_headers();

        let response = self
            .client
            .post_binary_with_headers(path, body, headers)
            .await?;

        let elapsed = start.elapsed();
        self.store_metadata(200, elapsed);

        Ok(response)
    }

    fn set_session_token(&mut self, token: Option<String>) {
        self.session_token = token;
    }

    fn set_timeout(&mut self, _timeout: Duration) {
        // This would require rebuilding the client
        // For now, we'll log this as a TODO
        eprintln!("TODO: Implement timeout modification for HttpClientWithProvider");
    }

    fn last_response_metadata(&self) -> Option<ResponseMetadata> {
        self.last_metadata.lock().ok()?.clone()
    }
}

// Alternative: Direct implementation on HttpClient
// This requires modifying the existing HttpClient struct
impl HttpClient {
    /// Get with custom headers (internal helper)
    pub(crate) async fn get_with_headers<T>(
        &self,
        path: &str,
        _headers: reqwest::header::HeaderMap,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        // This will be implemented by extending the existing get method
        // For now, delegate to existing get method
        self.get(path).await
    }

    /// Post with custom headers (internal helper)
    pub(crate) async fn post_with_headers<T, B>(
        &self,
        path: &str,
        body: &B,
        _headers: reqwest::header::HeaderMap,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        self.post(path, body).await
    }

    /// Put with custom headers (internal helper)
    pub(crate) async fn put_with_headers<T, B>(
        &self,
        path: &str,
        body: &B,
        _headers: reqwest::header::HeaderMap,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        self.put(path, body).await
    }

    /// Delete with custom headers (internal helper)
    pub(crate) async fn delete_with_headers(
        &self,
        path: &str,
        _headers: reqwest::header::HeaderMap,
    ) -> Result<()> {
        self.delete(path).await
    }

    /// Post binary with custom headers (internal helper)
    pub(crate) async fn post_binary_with_headers<B>(
        &self,
        path: &str,
        body: &B,
        _headers: reqwest::header::HeaderMap,
    ) -> Result<Vec<u8>>
    where
        B: Serialize + ?Sized,
    {
        self.post_binary(path, body).await
    }
}
