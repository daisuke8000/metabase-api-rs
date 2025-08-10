//! Authentication management module
//!
//! Handles session management and authentication state with secure memory management

use crate::core::models::User;
use secrecy::{ExposeSecret, Secret, SecretString};
use std::time::{Duration, Instant};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Secure session token wrapper with automatic memory clearing
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct SecureToken {
    #[zeroize(skip)]
    expires_at: Option<Instant>,
    token: String,
}

impl SecureToken {
    /// Creates a new secure token with optional expiry
    pub fn new(token: String, ttl: Option<Duration>) -> Self {
        let expires_at = ttl.map(|duration| Instant::now() + duration);
        Self { token, expires_at }
    }

    /// Checks if the token is expired
    pub fn is_expired(&self) -> bool {
        self.expires_at
            .is_some_and(|expiry| Instant::now() > expiry)
    }

    /// Gets the token if not expired
    pub fn get_if_valid(&self) -> Option<&str> {
        if self.is_expired() {
            None
        } else {
            Some(&self.token)
        }
    }
}

impl std::fmt::Debug for SecureToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SecureToken")
            .field("token", &"[REDACTED]")
            .field("expires_at", &self.expires_at)
            .finish()
    }
}

/// Manages authentication state and session tokens with enhanced security
#[derive(Debug, Clone)]
pub struct AuthManager {
    session_token: Option<SecureToken>,
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

    /// Checks if the user is authenticated with a valid, non-expired token
    pub fn is_authenticated(&self) -> bool {
        self.session_token
            .as_ref()
            .is_some_and(|token| !token.is_expired())
    }

    /// Gets the current session token if valid and not expired
    pub fn session_token(&self) -> Option<&str> {
        self.session_token
            .as_ref()
            .and_then(|token| token.get_if_valid())
    }
    
    /// Gets the session ID (same as session token)
    pub fn get_session_id(&self) -> Option<String> {
        self.session_token()
            .map(|s| s.to_string())
    }

    /// Gets the current user information
    pub fn current_user(&self) -> Option<&User> {
        self.current_user.as_ref()
    }

    /// Sets the session information with secure token storage
    pub fn set_session(&mut self, token: String, user: User) {
        self.set_session_with_ttl(token, user, None);
    }

    /// Sets the session information with token expiry
    pub fn set_session_with_ttl(&mut self, token: String, user: User, ttl: Option<Duration>) {
        self.session_token = Some(SecureToken::new(token, ttl));
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

/// Authentication credentials with secure password handling
#[derive(Clone)]
pub enum Credentials {
    /// Email and password authentication
    EmailPassword {
        email: String,
        password: SecretString,
    },
    /// API key authentication  
    ApiKey { key: Secret<String> },
}

impl Credentials {
    /// Creates new email/password credentials
    pub fn email_password(email: impl Into<String>, password: impl Into<String>) -> Self {
        Self::EmailPassword {
            email: email.into(),
            password: SecretString::new(password.into()),
        }
    }

    /// Creates new API key credentials
    pub fn new_api_key(key: impl Into<String>) -> Self {
        Self::ApiKey {
            key: Secret::new(key.into()),
        }
    }

    /// Gets email if this is email/password credentials
    pub fn email(&self) -> Option<&str> {
        match self {
            Self::EmailPassword { email, .. } => Some(email),
            _ => None,
        }
    }

    /// Gets password if this is email/password credentials
    pub fn password(&self) -> Option<&str> {
        match self {
            Self::EmailPassword { password, .. } => Some(password.expose_secret()),
            _ => None,
        }
    }

    /// Gets API key if this is API key credentials
    pub fn api_key(&self) -> Option<&str> {
        match self {
            Self::ApiKey { key } => Some(key.expose_secret()),
            _ => None,
        }
    }
}

impl std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmailPassword { email, .. } => f
                .debug_struct("EmailPassword")
                .field("email", email)
                .field("password", &"[REDACTED]")
                .finish(),
            Self::ApiKey { .. } => f
                .debug_struct("ApiKey")
                .field("key", &"[REDACTED]")
                .finish(),
        }
    }
}
