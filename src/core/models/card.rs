use super::common::MetabaseId;
use super::parameter::{Parameter, ParameterMapping};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Card type enumeration as per Metabase API specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum CardType {
    #[default]
    Question,
    Metric,
    Model,
}

/// Query type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum QueryType {
    #[default]
    Query,
    Native,
}

/// Represents a Metabase Card (saved question)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: MetabaseId,
    pub name: String,
    /// Required field as per API specification
    #[serde(rename = "type", default)]
    pub card_type: CardType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<MetabaseId>,
    #[serde(default = "default_display")]
    pub display: String,
    #[serde(default)]
    pub visualization_settings: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_query: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub archived: bool,
    #[serde(default)]
    pub enable_embedding: bool,
    #[serde(default)]
    pub embedding_params: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_metadata: Option<Value>,
    // Fields verified from Metabase API documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<MetabaseId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<MetabaseId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<MetabaseId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_type: Option<QueryType>,
    // Additional fields from API specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_ttl: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_tab_id: Option<MetabaseId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<MetabaseId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub made_public_by_id: Option<MetabaseId>,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
    #[serde(default)]
    pub parameter_mappings: Vec<ParameterMapping>,
}

fn default_display() -> String {
    "table".to_string()
}

impl Card {
    /// Create a new Card with minimal required fields
    pub fn new(id: MetabaseId, name: String, card_type: CardType) -> Self {
        Self {
            id,
            name,
            card_type,
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
            creator_id: None,
            database_id: None,
            table_id: None,
            query_type: None,
            entity_id: None,
            cache_ttl: None,
            collection_position: None,
            dashboard_tab_id: None,
            dashboard_id: None,
            public_uuid: None,
            made_public_by_id: None,
            parameters: Vec::new(),
            parameter_mappings: Vec::new(),
        }
    }

    // Getters
    pub fn id(&self) -> MetabaseId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn card_type(&self) -> &CardType {
        &self.card_type
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
    card_type: CardType,
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
    creator_id: Option<MetabaseId>,
    database_id: Option<MetabaseId>,
    table_id: Option<MetabaseId>,
    query_type: Option<QueryType>,
    entity_id: Option<String>,
    cache_ttl: Option<i32>,
    collection_position: Option<i32>,
    dashboard_tab_id: Option<MetabaseId>,
    dashboard_id: Option<MetabaseId>,
    public_uuid: Option<String>,
    made_public_by_id: Option<MetabaseId>,
    parameters: Vec<Parameter>,
    parameter_mappings: Vec<ParameterMapping>,
}

impl CardBuilder {
    /// Create a new CardBuilder with required fields
    pub fn new(id: MetabaseId, name: String, card_type: CardType) -> Self {
        Self {
            id,
            name,
            card_type,
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
            creator_id: None,
            database_id: None,
            table_id: None,
            query_type: None,
            entity_id: None,
            cache_ttl: None,
            collection_position: None,
            dashboard_tab_id: None,
            dashboard_id: None,
            public_uuid: None,
            made_public_by_id: None,
            parameters: Vec::new(),
            parameter_mappings: Vec::new(),
        }
    }

    /// Create a new CardBuilder for creating a new card (ID will be assigned by server)
    pub fn new_card(name: impl Into<String>) -> Self {
        Self::new(MetabaseId(0), name.into(), CardType::default())
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

    pub fn card_type(mut self, card_type: CardType) -> Self {
        self.card_type = card_type;
        self
    }

    pub fn entity_id<S: Into<String>>(mut self, id: S) -> Self {
        self.entity_id = Some(id.into());
        self
    }

    pub fn cache_ttl(mut self, ttl: i32) -> Self {
        self.cache_ttl = Some(ttl);
        self
    }

    pub fn collection_position(mut self, position: i32) -> Self {
        self.collection_position = Some(position);
        self
    }

    pub fn dashboard_tab_id(mut self, id: MetabaseId) -> Self {
        self.dashboard_tab_id = Some(id);
        self
    }

    pub fn dashboard_id(mut self, id: MetabaseId) -> Self {
        self.dashboard_id = Some(id);
        self
    }

    pub fn parameters(mut self, params: Vec<Parameter>) -> Self {
        self.parameters = params;
        self
    }

    pub fn parameter_mappings(mut self, mappings: Vec<ParameterMapping>) -> Self {
        self.parameter_mappings = mappings;
        self
    }

    pub fn creator_id(mut self, id: MetabaseId) -> Self {
        self.creator_id = Some(id);
        self
    }

    pub fn database_id(mut self, id: MetabaseId) -> Self {
        self.database_id = Some(id);
        self
    }

    pub fn table_id(mut self, id: MetabaseId) -> Self {
        self.table_id = Some(id);
        self
    }

    pub fn query_type(mut self, query_type: QueryType) -> Self {
        self.query_type = Some(query_type);
        self
    }

    pub fn public_uuid<S: Into<String>>(mut self, uuid: S) -> Self {
        self.public_uuid = Some(uuid.into());
        self
    }

    pub fn made_public_by_id(mut self, id: MetabaseId) -> Self {
        self.made_public_by_id = Some(id);
        self
    }

    /// Build the Card instance
    pub fn build(self) -> Card {
        Card {
            id: self.id,
            name: self.name,
            card_type: self.card_type,
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
            creator_id: self.creator_id,
            database_id: self.database_id,
            table_id: self.table_id,
            query_type: self.query_type,
            entity_id: self.entity_id,
            cache_ttl: self.cache_ttl,
            collection_position: self.collection_position,
            dashboard_tab_id: self.dashboard_tab_id,
            dashboard_id: self.dashboard_id,
            public_uuid: self.public_uuid,
            made_public_by_id: self.made_public_by_id,
            parameters: self.parameters,
            parameter_mappings: self.parameter_mappings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::parameter::{ParameterTarget, VariableTarget};

    #[test]
    fn test_card_creation() {
        let card = Card::new(
            MetabaseId::new(1),
            "Test Card".to_string(),
            CardType::Question,
        );

        assert_eq!(card.id(), MetabaseId::new(1));
        assert_eq!(card.name(), "Test Card");
        assert_eq!(card.card_type(), &CardType::Question);
        assert!(card.description().is_none());
        assert!(card.collection_id().is_none());
    }

    #[test]
    fn test_card_with_builder() {
        let card = CardBuilder::new(
            MetabaseId::new(2),
            "Builder Card".to_string(),
            CardType::Metric,
        )
        .description("A test card created with builder")
        .collection_id(MetabaseId::new(10))
        .display("table")
        .cache_ttl(300)
        .build();

        assert_eq!(card.id(), MetabaseId::new(2));
        assert_eq!(card.name(), "Builder Card");
        assert_eq!(card.card_type(), &CardType::Metric);
        assert_eq!(card.description(), Some("A test card created with builder"));
        assert_eq!(card.collection_id(), Some(MetabaseId::new(10)));
        assert_eq!(card.display(), "table");
        assert_eq!(card.cache_ttl, Some(300));
    }

    #[test]
    fn test_card_deserialize_from_json() {
        let json_str = r#"{
            "id": 123,
            "name": "Sales Dashboard Card",
            "type": "question",
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
            "enable_embedding": true,
            "cache_ttl": 600,
            "entity_id": "abc123",
            "creator_id": 10,
            "database_id": 1,
            "table_id": 42,
            "query_type": "native",
            "public_uuid": "1234-5678-9012",
            "made_public_by_id": 15
        }"#;

        let card: Card = serde_json::from_str(json_str).unwrap();

        assert_eq!(card.id().as_i64(), 123);
        assert_eq!(card.name(), "Sales Dashboard Card");
        assert_eq!(card.card_type(), &CardType::Question);
        assert_eq!(card.description(), Some("Monthly sales overview"));
        assert_eq!(card.collection_id(), Some(MetabaseId::new(5)));
        assert_eq!(card.display(), "line");
        assert!(!card.archived());
        assert!(card.enable_embedding());
        assert_eq!(card.cache_ttl, Some(600));
        assert_eq!(card.entity_id, Some("abc123".to_string()));
        assert_eq!(card.creator_id, Some(MetabaseId::new(10)));
        assert_eq!(card.database_id, Some(MetabaseId::new(1)));
        assert_eq!(card.table_id, Some(MetabaseId::new(42)));
        assert_eq!(card.query_type, Some(QueryType::Native));
        assert_eq!(card.public_uuid, Some("1234-5678-9012".to_string()));
        assert_eq!(card.made_public_by_id, Some(MetabaseId::new(15)));
    }

    #[test]
    fn test_card_type_serialization() {
        assert_eq!(
            serde_json::to_string(&CardType::Question).unwrap(),
            r#""question""#
        );
        assert_eq!(
            serde_json::to_string(&CardType::Metric).unwrap(),
            r#""metric""#
        );
        assert_eq!(
            serde_json::to_string(&CardType::Model).unwrap(),
            r#""model""#
        );
    }

    #[test]
    fn test_query_type_serialization() {
        assert_eq!(
            serde_json::to_string(&QueryType::Query).unwrap(),
            r#""query""#
        );
        assert_eq!(
            serde_json::to_string(&QueryType::Native).unwrap(),
            r#""native""#
        );
    }

    #[test]
    fn test_card_with_new_fields() {
        let card = CardBuilder::new(
            MetabaseId::new(100),
            "Analytics Card".to_string(),
            CardType::Question,
        )
        .description("Advanced analytics")
        .database_id(MetabaseId::new(1))
        .table_id(MetabaseId::new(5))
        .query_type(QueryType::Native)
        .creator_id(MetabaseId::new(42))
        .public_uuid("uuid-1234")
        .made_public_by_id(MetabaseId::new(10))
        .build();

        assert_eq!(card.database_id, Some(MetabaseId::new(1)));
        assert_eq!(card.table_id, Some(MetabaseId::new(5)));
        assert_eq!(card.query_type, Some(QueryType::Native));
        assert_eq!(card.creator_id, Some(MetabaseId::new(42)));
        assert_eq!(card.public_uuid, Some("uuid-1234".to_string()));
        assert_eq!(card.made_public_by_id, Some(MetabaseId::new(10)));
    }

    #[test]
    fn test_card_with_parameters() {
        let parameter = Parameter {
            id: "date_param".to_string(),
            param_type: "date/relative".to_string(),
            name: "Date Filter".to_string(),
            slug: "date".to_string(),
            default: Some(serde_json::json!("past7days")),
            required: false,
            options: None,
            values_source_type: None,
            values_source_config: None,
        };

        let parameter_mapping = ParameterMapping {
            parameter_id: "date_param".to_string(),
            card_id: 100,
            target: ParameterTarget::Variable(VariableTarget {
                target_type: "variable".to_string(),
                id: "start_date".to_string(),
            }),
        };

        let card = CardBuilder::new(
            MetabaseId::new(100),
            "Parameterized Card".to_string(),
            CardType::Question,
        )
        .parameters(vec![parameter.clone()])
        .parameter_mappings(vec![parameter_mapping.clone()])
        .build();

        assert_eq!(card.parameters.len(), 1);
        assert_eq!(card.parameters[0].id, "date_param");
        assert_eq!(card.parameter_mappings.len(), 1);
        assert_eq!(card.parameter_mappings[0].parameter_id, "date_param");
    }
}
