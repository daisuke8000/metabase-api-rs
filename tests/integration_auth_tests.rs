//! Authentication integration tests

use integration_common::{get_metabase_url, get_test_email, get_test_password, wait_for_metabase};
use metabase_api_rs::{api::Credentials, ClientBuilder};

mod integration_common;

#[tokio::test]
#[ignore] // Requires running Metabase instance
async fn test_email_password_authentication() {
    // Ensure Metabase is ready
    wait_for_metabase(30)
        .await
        .expect("Metabase should be running");

    // Create client
    let mut client = ClientBuilder::new(&get_metabase_url())
        .build()
        .expect("Failed to build client");

    // Test authentication
    let result = client
        .authenticate(Credentials::EmailPassword {
            email: get_test_email(),
            password: get_test_password(),
        })
        .await;

    assert!(result.is_ok(), "Authentication should succeed");
}

#[tokio::test]
#[ignore] // Requires running Metabase instance
async fn test_authentication_with_invalid_credentials() {
    wait_for_metabase(30)
        .await
        .expect("Metabase should be running");

    let mut client = ClientBuilder::new(&get_metabase_url())
        .build()
        .expect("Failed to build client");

    let result = client
        .authenticate(Credentials::EmailPassword {
            email: "invalid@example.com".to_string(),
            password: "wrong_password".to_string(),
        })
        .await;

    assert!(
        result.is_err(),
        "Authentication should fail with invalid credentials"
    );
}

#[tokio::test]
#[ignore] // Requires running Metabase instance
async fn test_get_current_user() {
    wait_for_metabase(30)
        .await
        .expect("Metabase should be running");

    let mut client = ClientBuilder::new(&get_metabase_url())
        .build()
        .expect("Failed to build client");

    // Authenticate first
    client
        .authenticate(Credentials::EmailPassword {
            email: get_test_email(),
            password: get_test_password(),
        })
        .await
        .expect("Authentication should succeed");

    // Get current user
    let user = client.get_current_user().await;

    assert!(user.is_ok(), "Should be able to get current user");
    let user = user.unwrap();
    assert_eq!(user.email, get_test_email());
}

#[tokio::test]
#[ignore] // Requires running Metabase instance
async fn test_logout() {
    wait_for_metabase(30)
        .await
        .expect("Metabase should be running");

    let mut client = ClientBuilder::new(&get_metabase_url())
        .build()
        .expect("Failed to build client");

    // Authenticate
    client
        .authenticate(Credentials::EmailPassword {
            email: get_test_email(),
            password: get_test_password(),
        })
        .await
        .expect("Authentication should succeed");

    // Logout
    let result = client.logout().await;
    assert!(result.is_ok(), "Logout should succeed");

    // Verify we can't access protected resources
    let user_result = client.get_current_user().await;
    assert!(
        user_result.is_err(),
        "Should not be able to get user after logout"
    );
}
