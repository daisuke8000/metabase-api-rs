use super::common::MetabaseId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a Metabase Collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    id: MetabaseId,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<MetabaseId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    personal_owner_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slug: Option<String>,
    #[serde(default)]
    archived: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_write: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<DateTime<Utc>>,
}

impl Collection {
    /// Create a new Collection with minimal required fields
    pub fn new(id: MetabaseId, name: String) -> Self {
        Self {
            id,
            name,
            description: None,
            color: None,
            parent_id: None,
            personal_owner_id: None,
            namespace: None,
            slug: None,
            archived: false,
            can_write: None,
            created_at: None,
            updated_at: None,
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

    pub fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    pub fn parent_id(&self) -> Option<MetabaseId> {
        self.parent_id
    }

    pub fn personal_owner_id(&self) -> Option<i64> {
        self.personal_owner_id
    }

    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }

    pub fn slug(&self) -> Option<&str> {
        self.slug.as_deref()
    }

    pub fn archived(&self) -> bool {
        self.archived
    }

    pub fn can_write(&self) -> Option<bool> {
        self.can_write
    }

    /// Check if this is a personal collection
    pub fn is_personal(&self) -> bool {
        self.personal_owner_id.is_some()
    }

    /// Check if this is the root collection
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none() && !self.is_personal()
    }
}

/// Builder for creating Collection instances
pub struct CollectionBuilder {
    id: MetabaseId,
    name: String,
    description: Option<String>,
    color: Option<String>,
    parent_id: Option<MetabaseId>,
    personal_owner_id: Option<i64>,
    namespace: Option<String>,
    slug: Option<String>,
    archived: bool,
    can_write: Option<bool>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl CollectionBuilder {
    /// Create a new CollectionBuilder with required fields
    pub fn new(id: MetabaseId, name: String) -> Self {
        Self {
            id,
            name,
            description: None,
            color: None,
            parent_id: None,
            personal_owner_id: None,
            namespace: None,
            slug: None,
            archived: false,
            can_write: None,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn description<S: Into<String>>(mut self, desc: S) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn color<S: Into<String>>(mut self, color: S) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn parent_id(mut self, id: MetabaseId) -> Self {
        self.parent_id = Some(id);
        self
    }

    pub fn personal_owner_id(mut self, id: i64) -> Self {
        self.personal_owner_id = Some(id);
        self
    }

    pub fn namespace<S: Into<String>>(mut self, ns: S) -> Self {
        self.namespace = Some(ns.into());
        self
    }

    pub fn slug<S: Into<String>>(mut self, slug: S) -> Self {
        self.slug = Some(slug.into());
        self
    }

    pub fn archived(mut self, archived: bool) -> Self {
        self.archived = archived;
        self
    }

    pub fn can_write(mut self, can_write: bool) -> Self {
        self.can_write = Some(can_write);
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

    /// Build the Collection instance
    pub fn build(self) -> Collection {
        Collection {
            id: self.id,
            name: self.name,
            description: self.description,
            color: self.color,
            parent_id: self.parent_id,
            personal_owner_id: self.personal_owner_id,
            namespace: self.namespace,
            slug: self.slug,
            archived: self.archived,
            can_write: self.can_write,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_creation() {
        let collection = Collection::new(MetabaseId::new(1), "Test Collection".to_string());

        assert_eq!(collection.id(), MetabaseId::new(1));
        assert_eq!(collection.name(), "Test Collection");
        assert!(collection.description().is_none());
        assert!(collection.parent_id().is_none());
        assert!(!collection.is_personal());
    }

    #[test]
    fn test_collection_hierarchy() {
        let parent = Collection::new(MetabaseId::new(1), "Parent Collection".to_string());

        let child = CollectionBuilder::new(MetabaseId::new(2), "Child Collection".to_string())
            .parent_id(parent.id())
            .description("A child collection")
            .build();

        assert_eq!(child.parent_id(), Some(parent.id()));
        assert_eq!(child.description(), Some("A child collection"));
    }

    #[test]
    fn test_personal_collection() {
        let personal =
            CollectionBuilder::new(MetabaseId::new(100), "My Personal Collection".to_string())
                .personal_owner_id(42)
                .build();

        assert_eq!(personal.personal_owner_id(), Some(42));
        assert!(personal.is_personal());
    }
}
