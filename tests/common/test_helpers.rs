//! Common test helper functions and utilities
//!
//! Provides shared test utilities to ensure consistency across test suites

use metabase_api_rs::core::models::{User, common::UserId};
use chrono::Utc;

/// Creates a test user with all required fields
pub fn create_test_user() -> User {
    User {
        id: UserId(1),
        email: "test@example.com".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        is_superuser: false,
        is_active: true,
        is_qbnewb: false,
        date_joined: Utc::now(),
        last_login: Some(Utc::now()),
        common_name: None,
        group_ids: Vec::new(),
        locale: None,
        google_auth: false,
        ldap_auth: false,
        login_attributes: None,
        user_group_memberships: Vec::new(),
    }
}

/// Creates a test user with specific ID
pub fn create_test_user_with_id(id: i64) -> User {
    User {
        id: UserId(id),
        email: format!("test{}@example.com", id),
        first_name: "Test".to_string(),
        last_name: format!("User{}", id),
        is_superuser: false,
        is_active: true,
        is_qbnewb: false,
        date_joined: Utc::now(),
        last_login: Some(Utc::now()),
        common_name: None,
        group_ids: Vec::new(),
        locale: None,
        google_auth: false,
        ldap_auth: false,
        login_attributes: None,
        user_group_memberships: Vec::new(),
    }
}

/// Creates a superuser for testing admin functionality
pub fn create_test_superuser() -> User {
    User {
        id: UserId(999),
        email: "admin@example.com".to_string(),
        first_name: "Admin".to_string(),
        last_name: "User".to_string(),
        is_superuser: true,
        is_active: true,
        is_qbnewb: false,
        date_joined: Utc::now(),
        last_login: Some(Utc::now()),
        common_name: None,
        group_ids: Vec::new(),
        locale: None,
        google_auth: false,
        ldap_auth: false,
        login_attributes: None,
        user_group_memberships: Vec::new(),
    }
}