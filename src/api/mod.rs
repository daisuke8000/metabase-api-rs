//! API layer - Public client interface
//!
//! This module contains the public API for interacting with Metabase.

pub mod auth;
pub mod builder;
pub mod client;

// Re-export main types
pub use auth::{AuthManager, Credentials};
pub use builder::ClientBuilder;
pub use client::MetabaseClient;
