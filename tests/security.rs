//! Security test suite
//!
//! Comprehensive security testing for metabase-api-rs

#[path = "security/auth_security_tests.rs"]
mod auth_security_tests;

#[path = "security/http_security_tests.rs"]
mod http_security_tests;
