use crate::{Error, Result};
use reqwest::{Client, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use url::Url;

/// Validate URL protocol for security - only allow HTTP/HTTPS
fn validate_url_protocol(url: &str) -> Result<()> {
    let parsed =
        Url::parse(url).map_err(|e| Error::Config(format!("Invalid base URL '{}': {}", url, e)))?;

    match parsed.scheme() {
        "http" | "https" => Ok(()),
        scheme => Err(Error::Config(format!(
            "Unsupported protocol '{}' in URL '{}'. Only HTTP and HTTPS are allowed.",
            scheme, url
        ))),
    }
}

/// HTTP client for making requests to the Metabase API
#[derive(Debug, Clone)]
pub struct HttpClient {
    inner: Client,
    base_url: String,
}

impl HttpClient {
    /// Create a new HTTP client with default configuration
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        let base_url = base_url.into();

        // Validate base URL format and protocol
        validate_url_protocol(&base_url)?;

        let inner = Client::builder()
            .cookie_store(true)
            .timeout(Duration::from_secs(30))
            // Connection pooling configuration
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(10)
            .tcp_keepalive(Duration::from_secs(60))
            .build()
            .map_err(|e| Error::Network(e.to_string()))?;

        Ok(Self { inner, base_url })
    }

    /// Safely construct URL by joining base URL with path
    fn build_url(&self, path: &str) -> Result<Url> {
        let base = Url::parse(&self.base_url)
            .map_err(|e| Error::Config(format!("Invalid base URL: {}", e)))?;

        base.join(path)
            .map_err(|e| Error::Config(format!("Invalid URL path '{}': {}", path, e)))
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Make a GET request
    pub async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.build_url(path)?;
        let response = self
            .inner
            .get(url)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;

        self.handle_response(response).await
    }

    /// Make a POST request
    pub async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let url = self.build_url(path)?;
        let response = self
            .inner
            .post(url)
            .json(body)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;

        self.handle_response(response).await
    }

    /// Make a PUT request
    pub async fn put<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let url = self.build_url(path)?;
        let response = self
            .inner
            .put(url)
            .json(body)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;

        self.handle_response(response).await
    }

    /// Make a DELETE request
    pub async fn delete(&self, path: &str) -> Result<()> {
        let url = self.build_url(path)?;
        let response = self
            .inner
            .delete(url)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(Error::Http {
                status: status.as_u16(),
                message: error_text,
            })
        }
    }

    /// Make a POST request and return binary data
    pub async fn post_binary<B>(&self, path: &str, body: &B) -> Result<Vec<u8>>
    where
        B: Serialize + ?Sized,
    {
        let url = self.build_url(path)?;
        let response = self
            .inner
            .post(url)
            .json(body)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;

        self.handle_binary_response(response).await
    }

    /// Make a GET request and return binary data
    pub async fn get_binary(&self, path: &str) -> Result<Vec<u8>> {
        let url = self.build_url(path)?;
        let response = self
            .inner
            .get(url)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;

        self.handle_binary_response(response).await
    }

    /// Handle the response and convert to the expected type
    async fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();

        if status.is_success() {
            response
                .json::<T>()
                .await
                .map_err(|e| Error::Json(e.to_string()))
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            match status.as_u16() {
                401 => Err(Error::Authentication(error_text)),
                404 => Err(Error::NotFound(error_text)),
                429 => Err(Error::RateLimited { retry_after: None }),
                _ => Err(Error::Http {
                    status: status.as_u16(),
                    message: error_text,
                }),
            }
        }
    }

    /// Handle binary response
    async fn handle_binary_response(&self, response: Response) -> Result<Vec<u8>> {
        let status = response.status();

        if status.is_success() {
            response
                .bytes()
                .await
                .map(|b| b.to_vec())
                .map_err(|e| Error::Network(e.to_string()))
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            match status.as_u16() {
                401 => Err(Error::Authentication(error_text)),
                404 => Err(Error::NotFound(error_text)),
                429 => Err(Error::RateLimited { retry_after: None }),
                _ => Err(Error::Http {
                    status: status.as_u16(),
                    message: error_text,
                }),
            }
        }
    }
}

/// Builder for configuring an HTTP client
pub struct HttpClientBuilder {
    base_url: String,
    timeout: Duration,
    headers: Vec<(String, String)>,
    user_agent: Option<String>,
}

impl HttpClientBuilder {
    /// Create a new builder
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            timeout: Duration::from_secs(30),
            headers: Vec::new(),
            user_agent: None,
        }
    }

    /// Set the timeout duration
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Add a custom header
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    /// Set the user agent
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Build the HTTP client
    pub fn build(self) -> Result<HttpClient> {
        // Validate base URL format and protocol
        validate_url_protocol(&self.base_url)?;

        let mut client_builder = Client::builder().cookie_store(true).timeout(self.timeout);

        // Set user agent if provided
        if let Some(ua) = self.user_agent {
            client_builder = client_builder.user_agent(ua);
        }

        // Add custom headers
        if !self.headers.is_empty() {
            let mut headers = reqwest::header::HeaderMap::new();
            for (key, value) in self.headers {
                let header_name = reqwest::header::HeaderName::from_bytes(key.as_bytes())
                    .map_err(|e| Error::Network(format!("Invalid header name: {}", e)))?;
                let header_value = reqwest::header::HeaderValue::from_str(&value)
                    .map_err(|e| Error::Network(format!("Invalid header value: {}", e)))?;
                headers.insert(header_name, header_value);
            }
            client_builder = client_builder.default_headers(headers);
        }

        let inner = client_builder
            .build()
            .map_err(|e| Error::Network(e.to_string()))?;

        Ok(HttpClient {
            inner,
            base_url: self.base_url,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_client_builder() {
        let client = HttpClientBuilder::new("https://example.com")
            .timeout(Duration::from_secs(60))
            .header("x-api-key", "secret")
            .build();

        assert!(client.is_ok());
    }
}
