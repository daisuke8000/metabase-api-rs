//! Authentication management module
//!
//! Handles session management and authentication state

use crate::core::models::User;

/// Manages authentication state and session tokens
#[derive(Debug, Clone)]
pub struct AuthManager {
    session_token: Option<String>,
    current_user: Option<User>,
}

impl AuthManager {
    /// Creates a new AuthManager instance
    pub fn new() -> Self {
        Self {
            session_token: None,
            current_user: None,
        }
    }

    /// Checks if the user is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.session_token.is_some()
    }

    /// Gets the current session token
    pub fn session_token(&self) -> Option<&str> {
        self.session_token.as_deref()
    }

    /// Gets the current user information
    pub fn current_user(&self) -> Option<&User> {
        self.current_user.as_ref()
    }

    /// Sets the session information
    pub fn set_session(&mut self, token: String, user: User) {
        self.session_token = Some(token);
        self.current_user = Some(user);
    }

    /// Clears the session information
    pub fn clear_session(&mut self) {
        self.session_token = None;
        self.current_user = None;
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Authentication credentials
#[derive(Debug, Clone)]
pub enum Credentials {
    /// Email and password authentication
    EmailPassword { email: String, password: String },
    /// API key authentication
    ApiKey { key: String },
}
