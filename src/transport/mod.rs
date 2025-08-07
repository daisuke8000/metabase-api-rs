//! Transport layer for HTTP communication
//!
//! This module handles HTTP requests, responses, and retry logic

pub mod http;
pub mod retry;

// Re-export commonly used types
pub use http::{HttpClient, HttpClientBuilder};
pub use retry::{retry_with, RetryPolicy};
