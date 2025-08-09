//! API layer - Public client interface
//!
//! This module contains the public API for interacting with Metabase.

pub mod auth;
pub mod auth_adapter;
pub mod builder;
pub mod client;

use serde::{Deserialize, Serialize};

// Re-export main types
pub use auth::{AuthManager, Credentials};
pub use builder::ClientBuilder;
pub use client::MetabaseClient;

/// Parameters for listing cards
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardListParams {
    /// Filter parameter (e.g., "archived", "all")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f: Option<String>,

    /// Model type filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,

    /// Limit the number of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Offset for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    /// Include archived cards
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
}
