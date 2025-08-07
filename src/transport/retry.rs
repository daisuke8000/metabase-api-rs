use crate::{Error, Result};
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

/// Retry policy configuration
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    max_retries: u32,
    initial_delay: Duration,
    max_delay: Duration,
    backoff_factor: f64,
    with_jitter: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_factor: 2.0,
            with_jitter: false,
        }
    }
}

impl RetryPolicy {
    /// Create a new retry policy builder
    pub fn builder() -> RetryPolicyBuilder {
        RetryPolicyBuilder::default()
    }

    /// Get the maximum number of retries
    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }

    /// Get the initial delay
    pub fn initial_delay(&self) -> Duration {
        self.initial_delay
    }

    /// Get the maximum delay
    pub fn max_delay(&self) -> Duration {
        self.max_delay
    }

    /// Calculate the delay for a given retry attempt
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        let exponential_delay =
            self.initial_delay.as_millis() as f64 * self.backoff_factor.powi(attempt as i32);
        let capped_delay = exponential_delay.min(self.max_delay.as_millis() as f64);
        Duration::from_millis(capped_delay as u64)
    }

    /// Calculate delay with jitter
    pub fn calculate_delay_with_jitter(&self, attempt: u32) -> Duration {
        let base_delay = self.calculate_delay(attempt);

        if self.with_jitter {
            // Add random jitter between 75% and 125% of the base delay
            let jitter_factor = 0.75 + (rand::random::<f64>() * 0.5);
            let jittered_millis = (base_delay.as_millis() as f64 * jitter_factor) as u64;
            Duration::from_millis(jittered_millis)
        } else {
            base_delay
        }
    }
}

/// Builder for RetryPolicy
#[derive(Debug, Clone)]
pub struct RetryPolicyBuilder {
    max_retries: u32,
    initial_delay: Duration,
    max_delay: Duration,
    backoff_factor: f64,
    with_jitter: bool,
}

impl Default for RetryPolicyBuilder {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_factor: 2.0,
            with_jitter: false,
        }
    }
}

impl RetryPolicyBuilder {
    /// Set the maximum number of retries
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Set the initial delay
    pub fn initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    /// Set the maximum delay
    pub fn max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }

    /// Set the backoff factor
    pub fn backoff_factor(mut self, factor: f64) -> Self {
        self.backoff_factor = factor;
        self
    }

    /// Enable jitter
    pub fn with_jitter(mut self, enabled: bool) -> Self {
        self.with_jitter = enabled;
        self
    }

    /// Build the RetryPolicy
    pub fn build(self) -> RetryPolicy {
        RetryPolicy {
            max_retries: self.max_retries,
            initial_delay: self.initial_delay,
            max_delay: self.max_delay,
            backoff_factor: self.backoff_factor,
            with_jitter: self.with_jitter,
        }
    }
}

/// Execute an operation with retry logic
pub async fn retry_with<F, Fut, T>(policy: &RetryPolicy, mut operation: F) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let mut attempt = 0;
    let mut last_error = None;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                // Check if the error is retryable
                if !is_retryable(&error) {
                    return Err(error);
                }

                // Check if we've exceeded max retries
                if attempt >= policy.max_retries {
                    return Err(last_error.unwrap_or(error));
                }

                // Store the error for potential return
                last_error = Some(error);

                // Calculate delay and sleep
                let delay = if policy.with_jitter {
                    policy.calculate_delay_with_jitter(attempt)
                } else {
                    policy.calculate_delay(attempt)
                };

                sleep(delay).await;
                attempt += 1;
            }
        }
    }
}

/// Determine if an error is retryable
fn is_retryable(error: &Error) -> bool {
    match error {
        Error::Network(_) => true,
        Error::Http { status, .. } => {
            // Retry on server errors (5xx) and certain client errors
            *status >= 500 || *status == 408 || *status == 429
        }
        Error::RateLimited { .. } => true,
        Error::Server(_) => true,
        Error::Timeout => true,
        Error::Authentication(_) => false,
        Error::NotFound(_) => false,
        Error::Json(_) => false,
        Error::Validation(_) => false,
        Error::Config(_) => false,
        Error::Session(_) => false,
        Error::InvalidParameter(_) => false,
        Error::Unknown(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_policy_default() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.max_retries(), 3);
        assert_eq!(policy.initial_delay(), Duration::from_millis(100));
    }

    #[test]
    fn test_retry_policy_builder() {
        let policy = RetryPolicy::builder()
            .max_retries(5)
            .initial_delay(Duration::from_millis(200))
            .max_delay(Duration::from_secs(60))
            .backoff_factor(3.0)
            .with_jitter(true)
            .build();

        assert_eq!(policy.max_retries(), 5);
        assert_eq!(policy.initial_delay(), Duration::from_millis(200));
        assert_eq!(policy.max_delay(), Duration::from_secs(60));
    }

    #[test]
    fn test_calculate_delay() {
        let policy = RetryPolicy::builder()
            .initial_delay(Duration::from_millis(100))
            .backoff_factor(2.0)
            .build();

        assert_eq!(policy.calculate_delay(0), Duration::from_millis(100));
        assert_eq!(policy.calculate_delay(1), Duration::from_millis(200));
        assert_eq!(policy.calculate_delay(2), Duration::from_millis(400));
    }

    #[test]
    fn test_is_retryable() {
        assert!(is_retryable(&Error::Network("timeout".to_string())));
        assert!(is_retryable(&Error::Http {
            status: 500,
            message: "server error".to_string()
        }));
        assert!(is_retryable(&Error::RateLimited { retry_after: None }));

        assert!(!is_retryable(&Error::Authentication("invalid".to_string())));
        assert!(!is_retryable(&Error::NotFound("not found".to_string())));
        assert!(!is_retryable(&Error::Json("parse error".to_string())));
    }
}
