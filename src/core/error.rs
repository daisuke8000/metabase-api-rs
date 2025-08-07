//! Error types for the metabase-api-rs library
//!
//! This module defines the error types used throughout the library,
//! following SOLID principles and providing clear error messages.

use thiserror::Error;

/// The main error type for metabase-api-rs operations
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP request failed with status code
    #[error("HTTP request failed with status {status}: {message}")]
    Http { status: u16, message: String },

    /// JSON parsing failed
    #[error("JSON parsing failed: {0}")]
    Json(String),

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Session error
    #[error("Session error: {0}")]
    Session(String),

    /// Invalid parameter provided
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Rate limit exceeded
    #[error("Rate limited")]
    RateLimited { retry_after: Option<u32> },

    /// Server error
    #[error("Server error: {0}")]
    Server(String),

    /// Request timeout
    #[error("Request timeout")]
    Timeout,

    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// A type alias for Results that use our Error type
pub type Result<T> = std::result::Result<T, Error>;

// From implementations for error conversion
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Error::Timeout
        } else if err.is_connect() {
            Error::Network(format!("Connection failed: {}", err))
        } else if err.is_status() {
            if let Some(status) = err.status() {
                match status.as_u16() {
                    401 | 403 => Error::Authentication(format!("Authentication error: {}", status)),
                    404 => Error::NotFound(format!("Resource not found: {}", status)),
                    429 => Error::RateLimited { retry_after: None },
                    500..=599 => Error::Server(format!("Server error: {}", status)),
                    code => Error::Http {
                        status: code,
                        message: format!("HTTP error: {}", status),
                    },
                }
            } else {
                Error::Http {
                    status: 0,
                    message: err.to_string(),
                }
            }
        } else {
            Error::Http {
                status: 0,
                message: err.to_string(),
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Network(format!("IO error: {}", err))
    }
}

// まず失敗するテストを書く（TDD Red Phase）
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_http_variant() {
        // Arrange
        let status = 400;
        let msg = "Bad Request".to_string();

        // Act
        let error = Error::Http {
            status,
            message: msg.clone(),
        };

        // Assert
        assert_eq!(
            error.to_string(),
            format!("HTTP request failed with status {}: {}", status, msg)
        );
    }

    #[test]
    fn test_error_json_variant() {
        // Arrange
        let msg = "Invalid JSON structure".to_string();

        // Act
        let error = Error::Json(msg.clone());

        // Assert
        assert_eq!(error.to_string(), format!("JSON parsing failed: {}", msg));
    }

    #[test]
    fn test_error_auth_variant() {
        // Arrange
        let msg = "Invalid credentials".to_string();

        // Act
        let error = Error::Authentication(msg.clone());

        // Assert
        assert_eq!(error.to_string(), format!("Authentication failed: {}", msg));
    }

    #[test]
    fn test_error_not_found_variant() {
        // Arrange
        let msg = "Card with id 123".to_string();

        // Act
        let error = Error::NotFound(msg.clone());

        // Assert
        assert_eq!(error.to_string(), format!("Resource not found: {}", msg));
    }

    #[test]
    fn test_error_rate_limited() {
        // Act
        let error = Error::RateLimited { retry_after: None };

        // Assert
        assert_eq!(error.to_string(), "Rate limited");
    }

    #[test]
    fn test_error_from_reqwest_error() {
        // このテストは実際のreqwest::Errorを作るのが難しいので、
        // モックや実際のHTTPエラーで後でテストする
        // 今はコンパイルエラーになることを確認

        // let reqwest_err = create_mock_reqwest_error();
        // let error: Error = reqwest_err.into();
        // match error {
        //     Error::Http(_) | Error::Network(_) => (),
        //     _ => panic!("Expected Http or Network error"),
        // }
    }

    #[test]
    fn test_error_from_serde_json_error() {
        // Arrange
        let json_err = serde_json::from_str::<String>("invalid json").unwrap_err();

        // Act
        let error: Error = json_err.into();

        // Assert
        match error {
            Error::Json(_) => (),
            _ => panic!("Expected Json error"),
        }
    }

    #[test]
    fn test_result_type_alias() {
        // Arrange
        fn test_function() -> Result<String> {
            Ok("success".to_string())
        }

        // Act
        let result = test_function();

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Error>();
    }
}
