use super::common::MetabaseId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a Metabase Card (saved question)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    id: MetabaseId,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_id: Option<MetabaseId>,
    #[serde(default = "default_display")]
    display: String,
    #[serde(default)]
    visualization_settings: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_query: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<DateTime<Utc>>,
    #[serde(default)]
    archived: bool,
    #[serde(default)]
    enable_embedding: bool,
    #[serde(default)]
    embedding_params: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result_metadata: Option<Value>,
}

fn default_display() -> String {
    "table".to_string()
}

impl Card {
    /// Create a new Card with minimal required fields
    pub fn new(id: MetabaseId, name: String) -> Self {
        Self {
            id,
            name,
            description: None,
            collection_id: None,
            display: default_display(),
            visualization_settings: Value::Object(serde_json::Map::new()),
            dataset_query: None,
            created_at: None,
            updated_at: None,
            archived: false,
            enable_embedding: false,
            embedding_params: Value::Object(serde_json::Map::new()),
            result_metadata: None,
        }
    }

    // Getters
    pub fn id(&self) -> MetabaseId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn collection_id(&self) -> Option<MetabaseId> {
        self.collection_id
    }

    pub fn display(&self) -> &str {
        &self.display
    }

    pub fn visualization_settings(&self) -> &Value {
        &self.visualization_settings
    }

    pub fn dataset_query(&self) -> Option<&Value> {
        self.dataset_query.as_ref()
    }

    pub fn archived(&self) -> bool {
        self.archived
    }

    pub fn enable_embedding(&self) -> bool {
        self.enable_embedding
    }
}

/// Builder for creating Card instances
pub struct CardBuilder {
    id: MetabaseId,
    name: String,
    description: Option<String>,
    collection_id: Option<MetabaseId>,
    display: String,
    visualization_settings: Value,
    dataset_query: Option<Value>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    archived: bool,
    enable_embedding: bool,
    embedding_params: Value,
    result_metadata: Option<Value>,
}

impl CardBuilder {
    /// Create a new CardBuilder with required fields
    pub fn new(id: MetabaseId, name: String) -> Self {
        Self {
            id,
            name,
            description: None,
            collection_id: None,
            display: default_display(),
            visualization_settings: Value::Object(serde_json::Map::new()),
            dataset_query: None,
            created_at: None,
            updated_at: None,
            archived: false,
            enable_embedding: false,
            embedding_params: Value::Object(serde_json::Map::new()),
            result_metadata: None,
        }
    }

    pub fn description<S: Into<String>>(mut self, desc: S) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn collection_id(mut self, id: MetabaseId) -> Self {
        self.collection_id = Some(id);
        self
    }

    pub fn display<S: Into<String>>(mut self, display: S) -> Self {
        self.display = display.into();
        self
    }

    pub fn visualization_settings(mut self, settings: Value) -> Self {
        self.visualization_settings = settings;
        self
    }

    pub fn dataset_query(mut self, query: Value) -> Self {
        self.dataset_query = Some(query);
        self
    }

    pub fn created_at(mut self, dt: DateTime<Utc>) -> Self {
        self.created_at = Some(dt);
        self
    }

    pub fn updated_at(mut self, dt: DateTime<Utc>) -> Self {
        self.updated_at = Some(dt);
        self
    }

    pub fn archived(mut self, archived: bool) -> Self {
        self.archived = archived;
        self
    }

    pub fn enable_embedding(mut self, enable: bool) -> Self {
        self.enable_embedding = enable;
        self
    }

    pub fn embedding_params(mut self, params: Value) -> Self {
        self.embedding_params = params;
        self
    }

    pub fn result_metadata(mut self, metadata: Value) -> Self {
        self.result_metadata = Some(metadata);
        self
    }

    /// Build the Card instance
    pub fn build(self) -> Card {
        Card {
            id: self.id,
            name: self.name,
            description: self.description,
            collection_id: self.collection_id,
            display: self.display,
            visualization_settings: self.visualization_settings,
            dataset_query: self.dataset_query,
            created_at: self.created_at,
            updated_at: self.updated_at,
            archived: self.archived,
            enable_embedding: self.enable_embedding,
            embedding_params: self.embedding_params,
            result_metadata: self.result_metadata,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card::new(MetabaseId::new(1), "Test Card".to_string());

        assert_eq!(card.id(), MetabaseId::new(1));
        assert_eq!(card.name(), "Test Card");
        assert!(card.description().is_none());
        assert!(card.collection_id().is_none());
    }

    #[test]
    fn test_card_with_builder() {
        let card = CardBuilder::new(MetabaseId::new(2), "Builder Card".to_string())
            .description("A test card created with builder")
            .collection_id(MetabaseId::new(10))
            .display("table")
            .build();

        assert_eq!(card.id(), MetabaseId::new(2));
        assert_eq!(card.name(), "Builder Card");
        assert_eq!(card.description(), Some("A test card created with builder"));
        assert_eq!(card.collection_id(), Some(MetabaseId::new(10)));
        assert_eq!(card.display(), "table");
    }

    #[test]
    fn test_card_deserialize_from_json() {
        let json_str = r#"{
            "id": 123,
            "name": "Sales Dashboard Card",
            "description": "Monthly sales overview",
            "collection_id": 5,
            "display": "line",
            "visualization_settings": {
                "graph.dimensions": ["date"],
                "graph.metrics": ["count"]
            },
            "created_at": "2023-08-08T10:00:00Z",
            "updated_at": "2023-08-08T12:00:00Z",
            "archived": false,
            "enable_embedding": true
        }"#;

        let card: Card = serde_json::from_str(json_str).unwrap();

        assert_eq!(card.id().as_i64(), 123);
        assert_eq!(card.name(), "Sales Dashboard Card");
        assert_eq!(card.description(), Some("Monthly sales overview"));
        assert_eq!(card.collection_id(), Some(MetabaseId::new(5)));
        assert_eq!(card.display(), "line");
        assert!(!card.archived());
        assert!(card.enable_embedding());
    }
}
