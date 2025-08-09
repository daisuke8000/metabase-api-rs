//! Dashboard model representing Metabase dashboards
//!
//! This module provides the core data structures for working with
//! Metabase dashboards, including dashboard cards and parameters.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::common::{DashboardId, UserId};

/// Represents a Metabase dashboard
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dashboard {
    /// Unique identifier for the dashboard
    pub id: Option<DashboardId>,

    /// Dashboard name
    pub name: String,

    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// ID of the collection this dashboard belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<i32>,

    /// Creator of the dashboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<UserId>,

    /// Dashboard parameters for filtering
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<DashboardParameter>,

    /// Cards (visualizations) on the dashboard
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cards: Vec<DashboardCard>,

    /// When the dashboard was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// When the dashboard was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

    /// Whether the dashboard is archived
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    // Additional fields from API specification
    /// Cache time-to-live in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_ttl: Option<i32>,

    /// Position in the collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_position: Option<i32>,

    /// Whether embedding is enabled
    #[serde(default)]
    pub enable_embedding: bool,

    /// Embedding parameters
    #[serde(default)]
    pub embedding_params: serde_json::Value,
}

/// Represents a parameter on a dashboard for filtering
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashboardParameter {
    /// Parameter ID
    pub id: String,

    /// Parameter name
    pub name: String,

    /// Parameter slug for URL
    pub slug: String,

    /// Parameter type (e.g., "category", "date", "number")
    #[serde(rename = "type")]
    pub parameter_type: String,

    /// Default value for the parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
}

/// Represents a card (visualization) on a dashboard
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashboardCard {
    /// Unique identifier for the dashboard card
    pub id: i32,

    /// ID of the card being displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<i32>,

    /// Position and size on the dashboard grid
    pub row: i32,
    pub col: i32,
    pub size_x: i32,
    pub size_y: i32,

    /// Visualization settings override
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visualization_settings: Option<serde_json::Value>,

    /// Parameter mappings
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter_mappings: Vec<ParameterMapping>,
}

/// Represents a parameter mapping between dashboard and card
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterMapping {
    /// Dashboard parameter ID
    pub parameter_id: String,

    /// Card parameter ID
    pub card_id: i32,

    /// Target field or variable
    pub target: serde_json::Value,
}

/// Request payload for creating a dashboard
#[derive(Debug, Clone, Serialize)]
pub struct CreateDashboardRequest {
    /// Dashboard name
    pub name: String,

    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Collection to place the dashboard in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<i32>,

    /// Initial parameters
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<DashboardParameter>,
}

/// Request payload for updating a dashboard
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateDashboardRequest {
    /// New name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// New description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// New collection ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<i32>,

    /// Whether to archive the dashboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    /// Updated parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<DashboardParameter>>,
}

impl Dashboard {
    /// Creates a new dashboard builder
    pub fn builder(name: impl Into<String>) -> DashboardBuilder {
        DashboardBuilder::new(name)
    }
}

/// Builder for creating Dashboard instances
pub struct DashboardBuilder {
    name: String,
    description: Option<String>,
    collection_id: Option<i32>,
    parameters: Vec<DashboardParameter>,
    cards: Vec<DashboardCard>,
    cache_ttl: Option<i32>,
    collection_position: Option<i32>,
    enable_embedding: bool,
    embedding_params: serde_json::Value,
}

impl DashboardBuilder {
    /// Creates a new dashboard builder with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            collection_id: None,
            parameters: Vec::new(),
            cards: Vec::new(),
            cache_ttl: None,
            collection_position: None,
            enable_embedding: false,
            embedding_params: serde_json::Value::Object(serde_json::Map::new()),
        }
    }

    /// Sets the dashboard description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Sets the collection ID
    pub fn collection_id(mut self, id: i32) -> Self {
        self.collection_id = Some(id);
        self
    }

    /// Adds a parameter to the dashboard
    pub fn add_parameter(mut self, param: DashboardParameter) -> Self {
        self.parameters.push(param);
        self
    }

    /// Adds a card to the dashboard
    pub fn add_card(mut self, card: DashboardCard) -> Self {
        self.cards.push(card);
        self
    }

    /// Sets the cache TTL
    pub fn cache_ttl(mut self, ttl: i32) -> Self {
        self.cache_ttl = Some(ttl);
        self
    }

    /// Sets the collection position
    pub fn collection_position(mut self, position: i32) -> Self {
        self.collection_position = Some(position);
        self
    }

    /// Sets whether embedding is enabled
    pub fn enable_embedding(mut self, enabled: bool) -> Self {
        self.enable_embedding = enabled;
        self
    }

    /// Sets the embedding parameters
    pub fn embedding_params(mut self, params: serde_json::Value) -> Self {
        self.embedding_params = params;
        self
    }

    /// Builds the Dashboard instance
    pub fn build(self) -> Dashboard {
        Dashboard {
            id: None, // Will be set by the server
            name: self.name,
            description: self.description,
            collection_id: self.collection_id,
            creator_id: None,
            parameters: self.parameters,
            cards: self.cards,
            created_at: None,
            updated_at: None,
            archived: Some(false),
            cache_ttl: self.cache_ttl,
            collection_position: self.collection_position,
            enable_embedding: self.enable_embedding,
            embedding_params: self.embedding_params,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_creation() {
        let dashboard = Dashboard::builder("Sales Dashboard")
            .description("Monthly sales metrics")
            .collection_id(10)
            .build();

        assert_eq!(dashboard.name, "Sales Dashboard");
        assert_eq!(
            dashboard.description,
            Some("Monthly sales metrics".to_string())
        );
        assert_eq!(dashboard.collection_id, Some(10));
        assert_eq!(dashboard.archived, Some(false));
        assert!(dashboard.parameters.is_empty());
        assert!(dashboard.cards.is_empty());
    }

    #[test]
    fn test_dashboard_with_parameters() {
        let param = DashboardParameter {
            id: "date_range".to_string(),
            name: "Date Range".to_string(),
            slug: "date_range".to_string(),
            parameter_type: "date/range".to_string(),
            default: None,
        };

        let dashboard = Dashboard::builder("Analytics Dashboard")
            .add_parameter(param.clone())
            .build();

        assert_eq!(dashboard.parameters.len(), 1);
        assert_eq!(dashboard.parameters[0].id, "date_range");
    }

    #[test]
    fn test_dashboard_card() {
        let card = DashboardCard {
            id: 1,
            card_id: Some(100),
            row: 0,
            col: 0,
            size_x: 4,
            size_y: 3,
            visualization_settings: None,
            parameter_mappings: Vec::new(),
        };

        assert_eq!(card.card_id, Some(100));
        assert_eq!(card.size_x, 4);
        assert_eq!(card.size_y, 3);
    }

    #[test]
    fn test_create_dashboard_request() {
        let request = CreateDashboardRequest {
            name: "New Dashboard".to_string(),
            description: Some("Test dashboard".to_string()),
            collection_id: Some(5),
            parameters: vec![],
        };

        assert_eq!(request.name, "New Dashboard");
        assert_eq!(request.description, Some("Test dashboard".to_string()));
        assert_eq!(request.collection_id, Some(5));
    }

    #[test]
    fn test_update_dashboard_request() {
        let request = UpdateDashboardRequest {
            name: Some("Updated Name".to_string()),
            archived: Some(true),
            ..Default::default()
        };

        assert_eq!(request.name, Some("Updated Name".to_string()));
        assert_eq!(request.archived, Some(true));
        assert!(request.description.is_none());
        assert!(request.collection_id.is_none());
    }

    #[test]
    fn test_dashboard_with_new_fields() {
        let dashboard = Dashboard::builder("Enhanced Dashboard")
            .cache_ttl(600)
            .collection_position(1)
            .enable_embedding(true)
            .embedding_params(serde_json::json!({"key": "value"}))
            .build();

        assert_eq!(dashboard.name, "Enhanced Dashboard");
        assert_eq!(dashboard.cache_ttl, Some(600));
        assert_eq!(dashboard.collection_position, Some(1));
        assert!(dashboard.enable_embedding);
        assert_eq!(dashboard.embedding_params["key"], "value");
    }
}
