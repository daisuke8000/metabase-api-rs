// Unit tests module

mod models_common_test;
mod models_card_test;
mod models_collection_test;
mod transport;

// API layer tests
mod api {
    mod test_client;
    mod test_builder;
    mod test_auth;
}

// Cache tests (only when cache feature is enabled)
#[cfg(feature = "cache")]
mod cache_tests;