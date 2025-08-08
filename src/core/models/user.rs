//! User and permission models for Metabase user management
//!
//! This module provides data structures for working with
//! Metabase users, groups, and permissions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::common::UserId;

/// Unique identifier for a group
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupId(pub i64);

/// Unique identifier for a permission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PermissionId(pub i64);

/// Represents a Metabase user
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for the user
    pub id: UserId,

    /// User's email address
    pub email: String,

    /// User's first name
    pub first_name: String,

    /// User's last name
    pub last_name: String,

    /// Common name (usually first_name + last_name)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,

    /// Whether the user is a superuser
    #[serde(default)]
    pub is_superuser: bool,

    /// Whether the user is active
    #[serde(default = "default_true")]
    pub is_active: bool,

    /// Whether the user is a Metabase internal user
    #[serde(default)]
    pub is_qbnewb: bool,

    /// When the user joined
    pub date_joined: DateTime<Utc>,

    /// Last login time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login: Option<DateTime<Utc>>,

    /// Groups the user belongs to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group_ids: Vec<GroupId>,

    /// User's locale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    /// Google auth enabled
    #[serde(default)]
    pub google_auth: bool,

    /// LDAP auth enabled
    #[serde(default)]
    pub ldap_auth: bool,

    // Additional fields from API specification
    /// Login attributes (e.g., SAML, LDAP attributes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_attributes: Option<serde_json::Value>,

    /// User group membership details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_group_memberships: Vec<UserGroupMembership>,
}

/// Represents user group membership details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserGroupMembership {
    /// Group ID
    pub id: GroupId,

    /// Whether this is the default group
    #[serde(default)]
    pub is_default: bool,
}

/// Health check status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Overall health status
    pub status: String,

    /// Metabase version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Database connection status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<bool>,
}

/// Represents a user group
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
    /// Unique identifier for the group
    pub id: GroupId,

    /// Group name
    pub name: String,

    /// Group members (user IDs)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<UserId>,
}

/// Represents a permission for a group
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Permission {
    /// Unique identifier for the permission
    pub id: PermissionId,

    /// Permission object path (e.g., "/db/1/schema/PUBLIC/")
    pub object: String,

    /// Group this permission applies to
    pub group_id: GroupId,
}

/// Permission graph for a group
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionGraph {
    /// Group ID
    pub group_id: GroupId,

    /// Native query permissions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<serde_json::Value>,

    /// Schema permissions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<serde_json::Value>,
}

/// Request to create a new user
#[derive(Debug, Clone, Serialize)]
pub struct CreateUserRequest {
    /// Email address
    pub email: String,

    /// First name
    pub first_name: String,

    /// Last name
    pub last_name: String,

    /// Password (required for email/password auth)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Group IDs to add the user to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group_ids: Vec<GroupId>,

    /// User's locale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

/// Request to update a user
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateUserRequest {
    /// New email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// New first name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// New last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// New password
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Whether user is active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,

    /// Whether user is superuser
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_superuser: Option<bool>,

    /// New group IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<GroupId>>,

    /// User's locale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

/// Request to create a new group
#[derive(Debug, Clone, Serialize)]
pub struct CreateGroupRequest {
    /// Group name
    pub name: String,

    /// Initial member IDs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<UserId>,
}

/// Request to update a group
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateGroupRequest {
    /// New name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// New member list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<UserId>>,
}

/// Membership change request
#[derive(Debug, Clone, Serialize)]
pub struct MembershipRequest {
    /// Group ID
    pub group_id: GroupId,

    /// User ID
    pub user_id: UserId,
}

fn default_true() -> bool {
    true
}

impl User {
    /// Creates a new user builder
    pub fn builder(
        email: impl Into<String>,
        first_name: impl Into<String>,
        last_name: impl Into<String>,
    ) -> UserBuilder {
        UserBuilder::new(email, first_name, last_name)
    }

    /// Gets the user's full name
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

/// Builder for creating User instances
pub struct UserBuilder {
    email: String,
    first_name: String,
    last_name: String,
    password: Option<String>,
    is_superuser: bool,
    group_ids: Vec<GroupId>,
    locale: Option<String>,
}

impl UserBuilder {
    /// Creates a new user builder
    pub fn new(
        email: impl Into<String>,
        first_name: impl Into<String>,
        last_name: impl Into<String>,
    ) -> Self {
        Self {
            email: email.into(),
            first_name: first_name.into(),
            last_name: last_name.into(),
            password: None,
            is_superuser: false,
            group_ids: Vec::new(),
            locale: None,
        }
    }

    /// Sets the password
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Sets whether the user is a superuser
    pub fn superuser(mut self, is_superuser: bool) -> Self {
        self.is_superuser = is_superuser;
        self
    }

    /// Adds a group ID
    pub fn add_group(mut self, group_id: GroupId) -> Self {
        self.group_ids.push(group_id);
        self
    }

    /// Sets the locale
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    /// Builds the User instance
    pub fn build(self) -> User {
        User {
            id: UserId(0), // Will be set by the server
            email: self.email,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            common_name: Some(format!("{} {}", self.first_name, self.last_name)),
            is_superuser: self.is_superuser,
            is_active: true,
            is_qbnewb: false,
            date_joined: Utc::now(),
            last_login: None,
            group_ids: self.group_ids,
            locale: self.locale,
            google_auth: false,
            ldap_auth: false,
            login_attributes: None,
            user_group_memberships: Vec::new(),
        }
    }

    /// Builds a CreateUserRequest
    pub fn build_request(self) -> CreateUserRequest {
        CreateUserRequest {
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
            password: self.password,
            group_ids: self.group_ids,
            locale: self.locale,
        }
    }
}

impl Group {
    /// Creates a new group builder
    pub fn builder(name: impl Into<String>) -> GroupBuilder {
        GroupBuilder::new(name)
    }
}

/// Builder for creating Group instances
pub struct GroupBuilder {
    name: String,
    members: Vec<UserId>,
}

impl GroupBuilder {
    /// Creates a new group builder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            members: Vec::new(),
        }
    }

    /// Adds a member to the group
    pub fn add_member(mut self, user_id: UserId) -> Self {
        self.members.push(user_id);
        self
    }

    /// Sets all members
    pub fn members(mut self, members: Vec<UserId>) -> Self {
        self.members = members;
        self
    }

    /// Builds the Group instance
    pub fn build(self) -> Group {
        Group {
            id: GroupId(0), // Will be set by the server
            name: self.name,
            members: self.members,
        }
    }

    /// Builds a CreateGroupRequest
    pub fn build_request(self) -> CreateGroupRequest {
        CreateGroupRequest {
            name: self.name,
            members: self.members,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::builder("john@example.com", "John", "Doe")
            .password("secure123")
            .superuser(false)
            .add_group(GroupId(1))
            .locale("en_US")
            .build();

        assert_eq!(user.email, "john@example.com");
        assert_eq!(user.first_name, "John");
        assert_eq!(user.last_name, "Doe");
        assert_eq!(user.full_name(), "John Doe");
        assert!(!user.is_superuser);
        assert_eq!(user.group_ids.len(), 1);
    }

    #[test]
    fn test_create_user_request() {
        let request = User::builder("jane@example.com", "Jane", "Smith")
            .password("password123")
            .add_group(GroupId(2))
            .build_request();

        assert_eq!(request.email, "jane@example.com");
        assert_eq!(request.first_name, "Jane");
        assert_eq!(request.last_name, "Smith");
        assert_eq!(request.password, Some("password123".to_string()));
        assert_eq!(request.group_ids.len(), 1);
    }

    #[test]
    fn test_group_creation() {
        let group = Group::builder("Administrators")
            .add_member(UserId(1))
            .add_member(UserId(2))
            .build();

        assert_eq!(group.name, "Administrators");
        assert_eq!(group.members.len(), 2);
    }

    #[test]
    fn test_update_user_request() {
        let request = UpdateUserRequest {
            email: Some("newemail@example.com".to_string()),
            is_active: Some(false),
            ..Default::default()
        };

        assert_eq!(request.email, Some("newemail@example.com".to_string()));
        assert_eq!(request.is_active, Some(false));
        assert!(request.first_name.is_none());
    }

    #[test]
    fn test_permission() {
        let permission = Permission {
            id: PermissionId(1),
            object: "/db/1/schema/PUBLIC/".to_string(),
            group_id: GroupId(1),
        };

        assert_eq!(permission.object, "/db/1/schema/PUBLIC/");
        assert_eq!(permission.group_id, GroupId(1));
    }
}
