//! Integration tests for trait implementations
//!
//! This module tests that all the trait abstractions work correctly
//! with their concrete implementations and adapters.

use metabase_api_rs::api::auth::{AuthManager, Credentials as ApiCredentials};
use metabase_api_rs::api::auth_adapter::{AuthManagerAdapter, AuthManagerAdapterBuilder};
use metabase_api_rs::core::models::User;
use metabase_api_rs::transport::{
    AuthProvider, AuthResponse, CacheProvider, HttpProvider, InMemoryCacheProvider,
    MockAuthProvider, MockCacheProvider,
};
use std::sync::Arc;
use std::time::Duration;

#[cfg(test)]
mod auth_integration_tests {
    use super::*;
    use metabase_api_rs::transport::Credentials as TransportCredentials;

    #[tokio::test]
    async fn test_auth_manager_adapter_with_mock() {
        // Create a mock HTTP auth provider
        let mut mock_provider = MockAuthProvider::default();
        mock_provider.should_succeed = true;
        mock_provider.mock_token = "integration_test_token".to_string();

        // Create an AuthManager
        let auth_manager = AuthManager::new();

        // Create the adapter
        let adapter = AuthManagerAdapter::new(auth_manager, Arc::new(mock_provider));

        // Test authentication flow
        let credentials = TransportCredentials::EmailPassword {
            email: "test@example.com".to_string(),
            password: "test_password".to_string(),
        };

        let auth_response = adapter.authenticate(&credentials).await.unwrap();
        assert_eq!(auth_response.session_token, "integration_test_token");
        assert!(auth_response.user.email.contains("@"));

        // Verify the AuthManager state was updated
        let inner = adapter.inner().await;
        assert!(inner.is_authenticated());
        assert_eq!(inner.session_token(), Some("integration_test_token"));
    }

    #[tokio::test]
    async fn test_auth_manager_adapter_session_management() {
        // Create a mock provider
        let mock_provider = Arc::new(MockAuthProvider::default());

        // Create an AuthManager
        let auth_manager = AuthManager::new();

        // Create the adapter
        let adapter = AuthManagerAdapter::new(auth_manager, mock_provider.clone());

        // Authenticate
        let credentials = TransportCredentials::ApiKey("api_key_123".to_string());
        let auth_response = adapter.authenticate(&credentials).await.unwrap();

        // Validate token
        let is_valid = adapter
            .validate_token(&auth_response.session_token)
            .await
            .unwrap();
        assert!(is_valid);

        // Refresh session
        let refreshed = adapter
            .refresh_session(&auth_response.session_token)
            .await
            .unwrap();
        assert_eq!(refreshed.session_token, auth_response.session_token);

        // Logout
        adapter.logout(&auth_response.session_token).await.unwrap();

        // Verify session is cleared
        let inner = adapter.inner().await;
        assert!(!inner.is_authenticated());
    }

    #[tokio::test]
    async fn test_auth_manager_adapter_builder() {
        let mock_provider = Arc::new(MockAuthProvider::default());

        let adapter = AuthManagerAdapterBuilder::new()
            .http_provider(mock_provider)
            .build()
            .unwrap();

        // Should work with default AuthManager
        let inner = adapter.inner().await;
        assert!(!inner.is_authenticated());
    }
}

#[cfg(test)]
mod cache_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_cache_provider() {
        let cache = InMemoryCacheProvider::new(100);

        // Test basic operations
        let key = "test_key".to_string();
        let value = "test_value".to_string();

        assert!(cache.set(&key, &value, None).await);
        let retrieved: Option<String> = cache.get(&key).await;
        assert_eq!(retrieved, Some(value));

        // Test TTL
        let ttl_key = "ttl_key".to_string();
        let ttl_value = "ttl_value".to_string();
        assert!(
            cache
                .set(&ttl_key, &ttl_value, Some(Duration::from_millis(50)))
                .await
        );
        assert!(cache.exists(&ttl_key).await);

        tokio::time::sleep(Duration::from_millis(100)).await;
        assert!(!cache.exists(&ttl_key).await);

        // Test stats
        let stats = cache.stats().await;
        assert!(stats.hits > 0 || stats.misses > 0);
    }

    #[tokio::test]
    async fn test_mock_cache_provider() {
        let mut cache = MockCacheProvider::default();
        cache.should_succeed = true;
        cache.enabled = true;

        let key = "mock_key".to_string();
        let value = 42u32;

        assert!(cache.set(&key, &value, None).await);
        assert!(cache.exists(&key).await);
        assert!(cache.remove(&key).await);
        assert!(cache.clear().await);

        let stats = cache.stats().await;
        assert_eq!(stats.hits, 80);
        assert_eq!(stats.misses, 20);
    }

    #[tokio::test]
    async fn test_cache_with_complex_types() {
        let cache = InMemoryCacheProvider::new(50);

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
        struct ComplexType {
            id: u64,
            name: String,
            tags: Vec<String>,
        }

        let key = "complex_key".to_string();
        let value = ComplexType {
            id: 123,
            name: "Test Object".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
        };

        assert!(cache.set(&key, &value, None).await);
        let retrieved: Option<ComplexType> = cache.get(&key).await;
        assert_eq!(retrieved, Some(value));
    }
}

#[cfg(test)]
mod http_provider_integration_tests {
    use super::*;
    use metabase_api_rs::transport::http_provider_impl::HttpClientWithProvider;

    #[tokio::test]
    async fn test_http_client_with_provider_creation() {
        // This test requires a valid URL but won't make actual requests
        let result = HttpClientWithProvider::new("http://localhost:3000");
        assert!(result.is_ok());

        let client = result.unwrap();
        
        // Test that we can set session token
        let mut client = client;
        client.set_session_token(Some("test_token".to_string()));
        
        // Note: We can't test actual HTTP calls without a running server
        // Those tests should be in integration tests with a mock server
    }
}

#[cfg(test)]
mod trait_composition_tests {
    use super::*;

    /// Test that traits can be composed together
    #[tokio::test]
    async fn test_trait_composition() {
        // Create providers
        let auth_provider = Arc::new(MockAuthProvider::default());
        let cache_provider = Arc::new(InMemoryCacheProvider::new(100));

        // Simulate a service that uses both traits
        async fn authenticate_and_cache<A, C>(
            auth: &A,
            cache: &C,
            email: &str,
            password: &str,
        ) -> metabase_api_rs::core::error::Result<String>
        where
            A: AuthProvider,
            C: CacheProvider,
        {
            use metabase_api_rs::transport::Credentials;

            // Authenticate
            let credentials = Credentials::EmailPassword {
                email: email.to_string(),
                password: password.to_string(),
            };
            let response = auth.authenticate(&credentials).await?;

            // Cache the session token
            let cache_key = format!("session:{}", email);
            cache
                .set(&cache_key, &response.session_token, Some(Duration::from_secs(3600)))
                .await;

            Ok(response.session_token)
        }

        // Test the composed function
        let token = authenticate_and_cache(
            auth_provider.as_ref(),
            cache_provider.as_ref(),
            "test@example.com",
            "password",
        )
        .await
        .unwrap();

        assert!(!token.is_empty());

        // Verify token was cached
        let cache_key = "session:test@example.com".to_string();
        let cached_token: Option<String> = cache_provider.get(&cache_key).await;
        assert_eq!(cached_token, Some(token));
    }
}

#[cfg(test)]
mod dependency_injection_tests {
    use super::*;

    /// Example service that uses dependency injection with traits
    struct MetabaseService {
        auth: Arc<dyn AuthProvider>,
        cache: Arc<dyn CacheProvider>,
    }

    impl MetabaseService {
        fn new(auth: Arc<dyn AuthProvider>, cache: Arc<dyn CacheProvider>) -> Self {
            Self { auth, cache }
        }

        async fn login(&self, email: &str, password: &str) -> Result<String, String> {
            use metabase_api_rs::transport::Credentials;

            // Check cache first
            let cache_key = format!("auth:{}", email);
            if let Some(token) = self.cache.get::<String>(&cache_key).await {
                return Ok(token);
            }

            // Authenticate if not cached
            let credentials = Credentials::EmailPassword {
                email: email.to_string(),
                password: password.to_string(),
            };

            let response = self
                .auth
                .authenticate(&credentials)
                .await
                .map_err(|e| e.to_string())?;

            // Cache the token
            self.cache
                .set(
                    &cache_key,
                    &response.session_token,
                    Some(Duration::from_secs(3600)),
                )
                .await;

            Ok(response.session_token)
        }
    }

    #[tokio::test]
    async fn test_service_with_dependency_injection() {
        // Inject mock implementations
        let auth = Arc::new(MockAuthProvider::default());
        let cache = Arc::new(MockCacheProvider::default());

        let service = MetabaseService::new(auth, cache);

        // Test login
        let token = service.login("user@example.com", "password").await.unwrap();
        assert_eq!(token, "mock_session_token_123");

        // Second login should use cache (though mock doesn't actually cache)
        let token2 = service.login("user@example.com", "password").await.unwrap();
        assert_eq!(token2, "mock_session_token_123");
    }

    #[tokio::test]
    async fn test_service_with_real_implementations() {
        // Inject real implementations
        let auth = Arc::new(MockAuthProvider::default()); // Would be HttpAuthProvider in production
        let cache = Arc::new(InMemoryCacheProvider::new(100));

        let service = MetabaseService::new(auth, cache);

        // Test login with caching
        let email = "real@example.com";
        let token1 = service.login(email, "password").await.unwrap();

        // Second login should use cache
        let cache_key = format!("auth:{}", email);
        let cached: Option<String> = cache.get(&cache_key).await;
        assert_eq!(cached, Some(token1.clone()));
    }
}