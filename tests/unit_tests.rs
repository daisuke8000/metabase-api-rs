//! Consolidated unit tests - Essential tests only

use metabase_api_rs::core::error::Error;

#[cfg(test)]
mod core_models {
    use super::*;

    #[test]
    fn test_error_handling() {
        let err = Error::Authentication("test".to_string());
        assert!(!err.to_string().is_empty());
    }
}

#[cfg(test)]
mod repository {
    use metabase_api_rs::repository::traits::RepositoryError;

    #[test]
    fn test_repository_error() {
        let err = RepositoryError::NotFound("test".to_string());
        assert!(err.to_string().contains("test"));
    }
}

#[cfg(test)]
mod service {
    use metabase_api_rs::service::traits::ServiceError;

    #[test]
    fn test_service_error() {
        let err = ServiceError::Validation("invalid".to_string());
        assert!(err.to_string().contains("invalid"));
    }
}
