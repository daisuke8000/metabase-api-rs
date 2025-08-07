//! # metabase-api-rs
//!
//! A simplified and efficient Rust client for the Metabase API.
//!
//! This library provides a clean interface to interact with Metabase's REST API,
//! with automatic authentication, retry logic, and optional caching.

pub mod api;
pub mod core;
pub mod transport;

// Re-export main types for convenience
pub use api::builder::ClientBuilder;
pub use api::client::MetabaseClient;
pub use core::error::{Error, Result};
// Re-export models
pub use core::models;

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
