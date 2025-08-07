use metabase_api_rs::transport::retry::{RetryPolicy, retry_with};
use metabase_api_rs::{Error, Result};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn test_retry_policy_creation() {
    let policy = RetryPolicy::default();
    assert_eq!(policy.max_retries(), 3);
    assert_eq!(policy.initial_delay(), Duration::from_millis(100));
}

#[tokio::test]
async fn test_retry_policy_builder() {
    let policy = RetryPolicy::builder()
        .max_retries(5)
        .initial_delay(Duration::from_millis(200))
        .max_delay(Duration::from_secs(10))
        .backoff_factor(3.0)
        .build();
    
    assert_eq!(policy.max_retries(), 5);
    assert_eq!(policy.initial_delay(), Duration::from_millis(200));
    assert_eq!(policy.max_delay(), Duration::from_secs(10));
}

#[tokio::test]
async fn test_successful_operation_no_retry() {
    let policy = RetryPolicy::default();
    let call_count = Arc::new(AtomicU32::new(0));
    let call_count_clone = call_count.clone();
    
    let result = retry_with(&policy, || {
        let count = call_count_clone.clone();
        async move {
            count.fetch_add(1, Ordering::SeqCst);
            Ok::<String, Error>("success".to_string())
        }
    }).await;
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
    assert_eq!(call_count.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn test_retry_on_failure() {
    let policy = RetryPolicy::builder()
        .max_retries(3)
        .initial_delay(Duration::from_millis(10))
        .build();
    
    let call_count = Arc::new(AtomicU32::new(0));
    let call_count_clone = call_count.clone();
    
    let result = retry_with(&policy, || {
        let count = call_count_clone.clone();
        async move {
            let current = count.fetch_add(1, Ordering::SeqCst);
            if current < 2 {
                // Fail first 2 attempts
                Err(Error::Network("Temporary failure".to_string()))
            } else {
                // Succeed on 3rd attempt
                Ok::<String, Error>("success after retry".to_string())
            }
        }
    }).await;
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success after retry");
    assert_eq!(call_count.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn test_max_retries_exceeded() {
    let policy = RetryPolicy::builder()
        .max_retries(2)
        .initial_delay(Duration::from_millis(10))
        .build();
    
    let call_count = Arc::new(AtomicU32::new(0));
    let call_count_clone = call_count.clone();
    
    let result = retry_with(&policy, || {
        let count = call_count_clone.clone();
        async move {
            count.fetch_add(1, Ordering::SeqCst);
            Err::<String, Error>(Error::Network("Always fails".to_string()))
        }
    }).await;
    
    assert!(result.is_err());
    // Should have been called: initial attempt + 2 retries = 3 times
    assert_eq!(call_count.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn test_non_retryable_error() {
    let policy = RetryPolicy::default();
    let call_count = Arc::new(AtomicU32::new(0));
    let call_count_clone = call_count.clone();
    
    let result = retry_with(&policy, || {
        let count = call_count_clone.clone();
        async move {
            count.fetch_add(1, Ordering::SeqCst);
            // Authentication error should not be retried
            Err::<String, Error>(Error::Authentication("Invalid credentials".to_string()))
        }
    }).await;
    
    assert!(result.is_err());
    // Should only be called once (no retries for auth errors)
    assert_eq!(call_count.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn test_exponential_backoff() {
    let policy = RetryPolicy::builder()
        .max_retries(3)
        .initial_delay(Duration::from_millis(100))
        .backoff_factor(2.0)
        .build();
    
    // Test delay calculation
    assert_eq!(policy.calculate_delay(0), Duration::from_millis(100));
    assert_eq!(policy.calculate_delay(1), Duration::from_millis(200));
    assert_eq!(policy.calculate_delay(2), Duration::from_millis(400));
}

#[tokio::test]
async fn test_max_delay_cap() {
    let policy = RetryPolicy::builder()
        .max_retries(10)
        .initial_delay(Duration::from_millis(100))
        .max_delay(Duration::from_millis(500))
        .backoff_factor(2.0)
        .build();
    
    // Even with exponential growth, should not exceed max_delay
    assert_eq!(policy.calculate_delay(5), Duration::from_millis(500));
    assert_eq!(policy.calculate_delay(10), Duration::from_millis(500));
}

#[tokio::test]
async fn test_jitter_addition() {
    let policy = RetryPolicy::builder()
        .max_retries(3)
        .initial_delay(Duration::from_millis(100))
        .with_jitter(true)
        .build();
    
    // With jitter, delay should be slightly randomized
    let delay = policy.calculate_delay_with_jitter(1);
    // Should be within reasonable range of expected delay
    assert!(delay >= Duration::from_millis(150)); // 200ms * 0.75
    assert!(delay <= Duration::from_millis(250)); // 200ms * 1.25
}