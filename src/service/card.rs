//! Card service implementation
//!
//! This module provides business logic for Card operations.

use super::traits::{Service, ServiceError, ServiceResult, ValidationContext};
use crate::core::models::{common::CardId, Card};
use crate::repository::{
    card::{CardFilterParams, CardRepository},
    traits::PaginationParams,
};
use async_trait::async_trait;
use std::sync::Arc;

/// Service trait for Card operations
#[async_trait]
pub trait CardService: Service {
    /// Get a card by ID
    async fn get_card(&self, id: CardId) -> ServiceResult<Card>;

    /// List cards with filters
    async fn list_cards(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<CardFilterParams>,
    ) -> ServiceResult<Vec<Card>>;

    /// Create a new card with validation
    async fn create_card(&self, card: Card) -> ServiceResult<Card>;

    /// Update a card with business rule validation
    async fn update_card(&self, id: CardId, card: Card) -> ServiceResult<Card>;

    /// Delete a card with cascade checks
    async fn delete_card(&self, id: CardId) -> ServiceResult<()>;

    /// Archive a card
    async fn archive_card(&self, id: CardId) -> ServiceResult<()>;

    /// Unarchive a card
    async fn unarchive_card(&self, id: CardId) -> ServiceResult<()>;

    /// Copy a card to a new collection
    async fn copy_card(
        &self,
        id: CardId,
        new_name: &str,
        collection_id: Option<i32>,
    ) -> ServiceResult<Card>;

    /// Validate card data
    async fn validate_card(&self, card: &Card) -> ServiceResult<()>;
}

/// HTTP implementation of CardService
pub struct HttpCardService {
    repository: Arc<dyn CardRepository>,
}

impl HttpCardService {
    /// Create a new HTTP card service
    pub fn new(repository: Arc<dyn CardRepository>) -> Self {
        Self { repository }
    }

    /// Validate card business rules
    fn validate_card_rules(&self, card: &Card) -> ServiceResult<()> {
        let mut context = ValidationContext::new();

        // Name validation
        if card.name.trim().is_empty() {
            context.add_error("Card name cannot be empty");
        }

        if card.name.len() > 255 {
            context.add_error("Card name cannot exceed 255 characters");
        }

        // Description validation
        if let Some(desc) = &card.description {
            if desc.len() > 5000 {
                context.add_error("Card description cannot exceed 5000 characters");
            }
        }

        // Validate dataset_query if present
        if let Some(query) = &card.dataset_query {
            if query.is_null() {
                context.add_error("Dataset query cannot be null");
            }
        }

        context.to_result()
    }
}

#[async_trait]
impl Service for HttpCardService {
    fn name(&self) -> &str {
        "CardService"
    }
}

#[async_trait]
impl CardService for HttpCardService {
    async fn get_card(&self, id: CardId) -> ServiceResult<Card> {
        self.repository.get(&id).await.map_err(ServiceError::from)
    }

    async fn list_cards(
        &self,
        pagination: Option<PaginationParams>,
        filters: Option<CardFilterParams>,
    ) -> ServiceResult<Vec<Card>> {
        self.repository
            .list_with_filters(pagination, filters)
            .await
            .map_err(ServiceError::from)
    }

    async fn create_card(&self, card: Card) -> ServiceResult<Card> {
        // Validate business rules
        self.validate_card_rules(&card)?;

        // Create via repository
        self.repository
            .create(&card)
            .await
            .map_err(ServiceError::from)
    }

    async fn update_card(&self, id: CardId, mut card: Card) -> ServiceResult<Card> {
        // Ensure ID matches
        card.id = Some(id);

        // Validate business rules
        self.validate_card_rules(&card)?;

        // Check if card exists
        self.repository.get(&id).await.map_err(ServiceError::from)?;

        // Update via repository
        self.repository
            .update(&id, &card)
            .await
            .map_err(ServiceError::from)
    }

    async fn delete_card(&self, id: CardId) -> ServiceResult<()> {
        // Check if card exists
        self.repository.get(&id).await.map_err(ServiceError::from)?;

        // TODO: Check for dependencies (dashboards using this card)

        // Delete via repository
        self.repository
            .delete(&id)
            .await
            .map_err(ServiceError::from)
    }

    async fn archive_card(&self, id: CardId) -> ServiceResult<()> {
        self.repository
            .archive(&id)
            .await
            .map_err(ServiceError::from)
    }

    async fn unarchive_card(&self, id: CardId) -> ServiceResult<()> {
        self.repository
            .unarchive(&id)
            .await
            .map_err(ServiceError::from)
    }

    async fn copy_card(
        &self,
        id: CardId,
        new_name: &str,
        _collection_id: Option<i32>,
    ) -> ServiceResult<Card> {
        // Validate new name
        if new_name.trim().is_empty() {
            return Err(ServiceError::Validation(
                "New card name cannot be empty".to_string(),
            ));
        }

        self.repository
            .copy(&id, new_name)
            .await
            .map_err(ServiceError::from)
    }

    async fn validate_card(&self, card: &Card) -> ServiceResult<()> {
        self.validate_card_rules(card)
    }
}
