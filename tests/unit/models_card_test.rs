#[cfg(test)]
mod tests {
    use metabase_api_rs::core::models::card::*;
    use metabase_api_rs::core::models::common::*;
    use serde_json::{json, Value};
    use chrono::{DateTime, Utc, TimeZone};

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
        let card = CardBuilder::new(MetabaseId::new(2), "Builder Card".to_string(), CardType::Model)
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
            "dataset_query": {
                "type": "native",
                "native": {
                    "query": "SELECT * FROM orders"
                }
            },
            "created_at": "2023-08-08T10:00:00Z",
            "updated_at": "2023-08-08T12:00:00Z",
            "archived": false,
            "enable_embedding": true,
            "embedding_params": {},
            "result_metadata": null
        }"#;

        let card: Card = serde_json::from_str(json_str).unwrap();
        
        assert_eq!(card.id().as_i64(), 123);
        assert_eq!(card.name(), "Sales Dashboard Card");
        assert_eq!(card.description(), Some("Monthly sales overview"));
        assert_eq!(card.collection_id(), Some(MetabaseId::new(5)));
        assert_eq!(card.display(), "line");
        assert!(!card.archived());
        assert!(card.enable_embedding());
        
        // Check visualization settings
        let viz_settings = card.visualization_settings();
        assert!(viz_settings["graph.dimensions"].is_array());
    }

    #[test]
    fn test_card_serialize_to_json() {
        let card = CardBuilder::new(MetabaseId::new(456), "Test Serialization".to_string(), CardType::Question)
            .description("Testing serialization")
            .display("bar")
            .build();
        
        let json = serde_json::to_value(&card).unwrap();
        
        assert_eq!(json["id"], 456);
        assert_eq!(json["name"], "Test Serialization");
        assert_eq!(json["description"], "Testing serialization");
        assert_eq!(json["display"], "bar");
    }

    #[test]
    fn test_card_display_types() {
        let display_types = vec![
            "table", "bar", "line", "area", "pie", 
            "scalar", "smartscalar", "gauge", "progress",
            "funnel", "waterfall", "map", "pivot"
        ];
        
        for display_type in display_types {
            let card = CardBuilder::new(MetabaseId::new(1), "Test".to_string(), CardType::Question)
                .display(display_type)
                .build();
            assert_eq!(card.display(), display_type);
        }
    }

    #[test]
    fn test_card_query_types() {
        // Native query
        let native_query = json!({
            "type": "native",
            "native": {
                "query": "SELECT COUNT(*) FROM users"
            }
        });
        
        let card = CardBuilder::new(MetabaseId::new(1), "Native Query Card".to_string(), CardType::Question)
            .dataset_query(native_query.clone())
            .build();
        
        assert_eq!(card.dataset_query(), Some(&native_query));
        
        // Structured query
        let structured_query = json!({
            "type": "query",
            "query": {
                "source-table": 2,
                "aggregation": [["count"]]
            }
        });
        
        let card2 = CardBuilder::new(MetabaseId::new(2), "Structured Query Card".to_string(), CardType::Question)
            .dataset_query(structured_query.clone())
            .build();
        
        assert_eq!(card2.dataset_query(), Some(&structured_query));
    }
}