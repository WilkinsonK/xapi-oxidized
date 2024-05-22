use std::{fmt::Debug, rc::Rc};

use oxinat_derive::uri_builder_alias;

use crate::{UriBuildError, UriBuilder, Version};

uri_builder_alias!(UserAdminUriBuilder);
ImplUserAdminUriBuilder! {
    (String),
}

macro_rules! username_is_some {
    () => {
        |this: &Self| this.parent.as_ref().is_some_and(|p| p.username.is_some())
    };
}

macro_rules! username_is_none {
    () => {
        |this: &Self| this.parent.as_ref().is_some_and(|p| p.username.is_none())
    };
}

/// Represents all URI endpoints for an admin to
/// manage XNAT users.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/users")]
#[match_path(path = "{parent}/users/{username}")]
pub struct UserUriBuilder<Parent>
where
    Parent: UserAdminUriBuilder,
{
    #[param]
    username: Option<String>,
    #[parent]
    parent: Option<Rc<Parent>>
}

/// Represents URI endpoints for an XNAT admin to
/// manage active users and their sessions.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/active", requires = "username_is_none!()")]
#[match_path(path = "{parent}/active/{username}", requires = "username_is_none!()")]
pub struct ActiveUriBuilder {
    #[param]
    username: Option<String>,
    #[parent]
    parent: Option<Rc<UserUriBuilder<String>>>
}

/// Represents URI endpoints for an XNAT admin to
/// manage whether a user is enabled or not.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/enabled", requires = "username_is_some!()")]
#[match_path(path = "{parent}/enabled/{flag}", requires = "username_is_some!()")]
pub struct EnabledUriBuilder {
    #[param]
    flag: Option<bool>,
    #[parent]
    parent: Option<Rc<UserUriBuilder<String>>>
}

/// Represents URI enpoints for an XNAT admin to
/// manage user groups.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/groups", requires = "username_is_some!()")]
#[match_path(path = "{parent}/groups/{group}", requires = "username_is_some!()")]
pub struct GroupsUriBuilder
{
    #[param]
    group: Option<String>,
    #[parent]
    parent: Option<Rc<UserUriBuilder<String>>>
}

/// Represents the URI endpoints for an XNAT admin
/// to access user profile metadata.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/profiles", requires = "username_is_none!()")]
#[match_path(path = "{parent}/profile/{username}", requires = "username_is_none!()")]
pub struct ProfilesUriBuilder {
    #[param]
    username: Option<String>,
    #[parent]
    parent: Option<Rc<UserUriBuilder<String>>>
}

/// Represents the URI endpoints for an XNAT admin
/// manage user roles.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/roles", requires = "username_is_some!()")]
#[match_path(path = "{parent}/roles/{role}", requires = "username_is_some!()")]
pub struct RolesUriBuilder {
    #[param]
    role: Option<String>,
    #[parent]
    parent: Option<Rc<UserUriBuilder<String>>>
}

/// Represents the URI endpoints for an XNAT admin
/// manage user roles.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/verified", requires = "username_is_some!()")]
#[match_path(path = "{parent}/verified/{flag}", requires = "username_is_some!()")]
pub struct VerifiedUriBuilder {
    #[param]
    flag: Option<bool>,
    #[parent]
    parent: Option<Rc<UserUriBuilder<String>>>
}

impl UserUriBuilder<String>
{
    /// Continue the builder into a
    /// `ActiveUriBuilder`.
    pub fn active(&self) -> ActiveUriBuilder {
        ActiveUriBuilder::from_parent(self.clone().into())
    }

    /// Produces the users/current URI endpoint.
    pub fn build_current(&self) -> anyhow::Result<String> {
        self.build_join("current")
    }

    /// Produces the users/projects URI endpoint.
    pub fn build_projects(&self) -> anyhow::Result<String> {
        if self.username.is_none() {
            self.build_join("projects")
        } else {
            Err(UriBuildError::Validation.into())
        }
    }

    /// Produces the users/username URI endpoint.
    pub fn build_username(&self) -> anyhow::Result<String> {
        if self.username.is_none() {
            self.build_join("username")
        } else {
            Err(UriBuildError::Validation.into())
        }
    }

    /// Continue the builder into an
    /// `EnabledUriBuilder`.
    pub fn enabled(&self) -> EnabledUriBuilder {
        EnabledUriBuilder::from_parent(self.clone().into())
    }

    /// Continue the builder into a
    /// `GroupsUriBuilder`.
    pub fn groups(&self) -> GroupsUriBuilder {
        GroupsUriBuilder::from_parent(self.clone().into())
    }

    /// Continue the builder into a
    /// `ProfilesUriBuilder`.
    pub fn profiles(&self) -> ProfilesUriBuilder {
        ProfilesUriBuilder::from_parent(self.clone().into())
    }

    /// Continue the builder into a
    /// `RolesUriBuilder`.
    pub fn roles(&self) -> RolesUriBuilder {
        RolesUriBuilder::from_parent(self.clone().into())
    }

    /// Continue the builder into a
    /// `VerifiedUriBuilder`.
    pub fn verified(&self) -> VerifiedUriBuilder {
        VerifiedUriBuilder::from_parent(self.clone().into())
    }
}

/// Represents the URI paths available for
/// endpoints meant for managing users.
pub trait UsersUri: Version {
    #[inline]
    fn users(&self) -> UserUriBuilder<String> {
        UserUriBuilder::from_parent(self.root_uri().into())
    }
}
