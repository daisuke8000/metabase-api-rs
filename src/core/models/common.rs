use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Metabase ID type - a newtype wrapper around i64
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MetabaseId(pub i64);

/// User ID type - a newtype wrapper around i64
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(pub i64);

impl UserId {
    /// Create a new UserId
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    /// Get the inner i64 value
    pub fn as_i64(&self) -> i64 {
        self.0
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl MetabaseId {
    /// Create a new MetabaseId
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    /// Get the inner i64 value
    pub fn as_i64(&self) -> i64 {
        self.0
    }
}

impl fmt::Display for MetabaseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Metabase DateTime type - a wrapper around `chrono::DateTime<Utc>`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MetabaseDateTime(DateTime<Utc>);

impl MetabaseDateTime {
    /// Create a new MetabaseDateTime
    pub fn new(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }

    /// Get the inner `DateTime<Utc>` value
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }
}

/// Pagination parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pagination {
    limit: usize,
    offset: usize,
}

impl Pagination {
    /// Create new pagination parameters
    pub fn new(limit: usize, offset: usize) -> Self {
        Self { limit, offset }
    }

    /// Create pagination for a specific page (1-indexed)
    pub fn with_page(limit: usize, page: usize) -> Self {
        let offset = if page > 0 { (page - 1) * limit } else { 0 };
        Self { limit, offset }
    }

    /// Get the limit
    pub fn limit(&self) -> usize {
        self.limit
    }

    /// Get the offset
    pub fn offset(&self) -> usize {
        self.offset
    }
}

/// Visibility enum for collections and other resources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Private,
    Limited,
}

/// Export format for query results
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    /// CSV format
    Csv,
    /// JSON format
    Json,
    /// Excel format
    Xlsx,
}

impl ExportFormat {
    /// Get the format as a string for API endpoints
    pub fn as_str(&self) -> &str {
        match self {
            Self::Csv => "csv",
            Self::Json => "json",
            Self::Xlsx => "xlsx",
        }
    }
}

impl fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn test_export_format() {
        // Test serialization
        let csv = ExportFormat::Csv;
        let json_str = serde_json::to_string(&csv).unwrap();
        assert_eq!(json_str, "\"csv\"");

        // Test deserialization
        let xlsx: ExportFormat = serde_json::from_str("\"xlsx\"").unwrap();
        assert_eq!(xlsx, ExportFormat::Xlsx);

        // Test as_str method
        assert_eq!(ExportFormat::Csv.as_str(), "csv");
        assert_eq!(ExportFormat::Json.as_str(), "json");
        assert_eq!(ExportFormat::Xlsx.as_str(), "xlsx");

        // Test Display trait
        assert_eq!(format!("{}", ExportFormat::Json), "json");
    }
}
