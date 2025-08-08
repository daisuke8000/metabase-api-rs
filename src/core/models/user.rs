//! User model definition

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a Metabase user
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// User's unique identifier
    pub id: i32,

    /// User's email address
    pub email: String,

    /// User's first name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// User's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// Whether the user is a superuser
    #[serde(default)]
    pub is_superuser: bool,

    /// Whether the user account is active
    #[serde(default = "default_true")]
    pub is_active: bool,

    /// When the user joined
    pub date_joined: DateTime<Utc>,

    /// Last login time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login: Option<DateTime<Utc>>,

    /// User's common name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
}

fn default_true() -> bool {
    true
}

/// Health check status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Overall status
    pub status: String,

    /// Database connection status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,

    /// Cache status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<String>,
}
