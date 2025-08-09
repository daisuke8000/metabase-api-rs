//! Transport layer for HTTP communication
//!
//! This module handles HTTP requests, responses, and retry logic

pub mod auth_provider_impl;
pub mod auth_traits;
pub mod cache_provider_impl;
pub mod cache_traits;
pub mod http;
pub mod http_provider_impl;
pub mod http_provider_safe;
pub mod retry;
pub mod traits;

// Re-export commonly used types
pub use auth_provider_impl::{HttpAuthProvider, HttpAuthProviderBuilder};
pub use auth_traits::{AuthProvider, AuthResponse, Credentials, MockAuthProvider};
pub use cache_provider_impl::{InMemoryCacheProvider, InMemoryCacheProviderBuilder};
pub use cache_traits::{
    CacheKey, CacheProvider, CacheStats, CompoundKey, MockCacheProvider, NoOpCacheProvider,
};
pub use http::{HttpClient, HttpClientBuilder};
pub use http_provider_safe::{HttpClientAdapter, HttpProviderExt, HttpProviderSafe};
pub use retry::{retry_with, RetryPolicy};
pub use traits::{HttpProvider, NoOpHttpProvider, ResponseMetadata};
