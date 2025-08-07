#[cfg(test)]
mod tests {
    use metabase_api_rs::core::models::common::*;
    use chrono::{DateTime, Utc};
    use serde_json;

    #[test]
    fn test_metabase_id_serialize_deserialize() {
        let id = MetabaseId::new(123);
        
        // Serialize
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "123");
        
        // Deserialize
        let deserialized: MetabaseId = serde_json::from_str("456").unwrap();
        assert_eq!(deserialized.as_i64(), 456);
    }

    #[test]
    fn test_metabase_id_display() {
        let id = MetabaseId::new(789);
        assert_eq!(format!("{}", id), "789");
    }

    #[test]
    fn test_metabase_datetime_serialize_deserialize() {
        let dt_str = "2023-08-08T10:30:00Z";
        let dt: MetabaseDateTime = serde_json::from_str(&format!("\"{}\"", dt_str)).unwrap();
        
        // Check inner value
        let inner: DateTime<Utc> = dt.into_inner();
        assert_eq!(inner.to_rfc3339(), "2023-08-08T10:30:00+00:00");
        
        // Serialize back
        let json = serde_json::to_string(&dt).unwrap();
        assert!(json.contains("2023-08-08"));
    }

    #[test]
    fn test_pagination() {
        let pagination = Pagination::new(10, 20);
        assert_eq!(pagination.limit(), 10);
        assert_eq!(pagination.offset(), 20);
        
        // Test with_page helper
        let page_2 = Pagination::with_page(50, 2);
        assert_eq!(page_2.limit(), 50);
        assert_eq!(page_2.offset(), 50); // Page 2 with 50 items per page
    }

    #[test]
    fn test_visibility_enum() {
        let public = Visibility::Public;
        let json = serde_json::to_string(&public).unwrap();
        assert_eq!(json, "\"public\"");
        
        let private: Visibility = serde_json::from_str("\"private\"").unwrap();
        assert_eq!(private, Visibility::Private);
    }
}