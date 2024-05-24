use std::{fmt::Debug, path::PathBuf, rc::Rc};

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
pub struct ActiveUriBuilder<'a> {
    #[param]
    username: Option<String>,
    #[parent]
    parent: Option<&'a UserUriBuilder<String>>
}

/// Represents URI endpoints for an XNAT admin to
/// manage whether a user is enabled or not.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/enabled", requires = "username_is_some!()")]
#[match_path(path = "{parent}/enabled/{flag}", requires = "username_is_some!()")]
pub struct EnabledUriBuilder<'a> {
    #[param]
    flag: Option<bool>,
    #[parent]
    parent: Option<&'a UserUriBuilder<String>>
}

/// Represents URI enpoints for an XNAT admin to
/// manage user groups.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/groups", requires = "username_is_some!()")]
#[match_path(path = "{parent}/groups/{group}", requires = "username_is_some!()")]
pub struct GroupsUriBuilder<'a>
{
    #[param]
    group: Option<String>,
    #[parent]
    parent: Option<&'a UserUriBuilder<String>>
}

/// Represents the URI endpoints for an XNAT admin
/// to access user profile metadata.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/profiles", requires = "username_is_none!()")]
#[match_path(path = "{parent}/profile/{username}", requires = "username_is_none!()")]
pub struct ProfilesUriBuilder<'a> {
    #[param]
    username: Option<String>,
    #[parent]
    parent: Option<&'a UserUriBuilder<String>>
}

/// Represents the URI endpoints for an XNAT admin
/// manage user roles.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/roles", requires = "username_is_some!()")]
#[match_path(path = "{parent}/roles/{role}", requires = "username_is_some!()")]
pub struct RolesUriBuilder<'a> {
    #[param]
    role: Option<String>,
    #[parent]
    parent: Option<&'a UserUriBuilder<String>>
}

/// Represents the URI endpoints for an XNAT admin
/// manage user roles.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/verified", requires = "username_is_some!()")]
#[match_path(path = "{parent}/verified/{flag}", requires = "username_is_some!()")]
pub struct VerifiedUriBuilder<'a> {
    #[param]
    flag: Option<bool>,
    #[parent]
    parent: Option<&'a UserUriBuilder<String>>
}

impl UserUriBuilder<String>
{
    /// Continue the builder into a
    /// `ActiveUriBuilder`.
    pub fn active(&self) -> ActiveUriBuilder {
        ActiveUriBuilder::from_parent(&Rc::new(self))
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
        EnabledUriBuilder::from_parent(&Rc::new(self))
    }

    /// Continue the builder into a
    /// `GroupsUriBuilder`.
    pub fn groups(&self) -> GroupsUriBuilder {
        GroupsUriBuilder::from_parent(&Rc::new(self))
    }

    /// Continue the builder into a
    /// `ProfilesUriBuilder`.
    pub fn profiles(&self) -> ProfilesUriBuilder {
        ProfilesUriBuilder::from_parent(&Rc::new(self))
    }

    /// Continue the builder into a
    /// `RolesUriBuilder`.
    pub fn roles(&self) -> RolesUriBuilder {
        RolesUriBuilder::from_parent(&Rc::new(self))
    }

    /// Continue the builder into a
    /// `VerifiedUriBuilder`.
    pub fn verified(&self) -> VerifiedUriBuilder {
        VerifiedUriBuilder::from_parent(&Rc::new(self))
    }
}

/// Represents the legacy URI endpoint paths
/// available for a XNAT admin to manage users.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/users")]
#[match_path(path = "{parent}/user/{username}")]
pub struct UsersUriLegacyBuilder<Parent>
where
    Parent: UserAdminUriBuilder,
{
    #[param]
    username: Option<String>,
    #[parent]
    parent: Option<Rc<Parent>>
}

/// Represents the legacy URI endpoint paths
/// available for access to user caches.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/cache", requires = "username_is_none!()")]
pub struct CacheUriLegacyBuilder<'a> {
    #[parent]
    parent: Option<&'a UsersUriLegacyBuilder<String>>
}

/// Represents the legacy URI endpoint paths
/// available for access to user cached resources.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/resources")]
#[match_path(path = "{parent}/resources/{folder}")]
#[match_path(path = "{parent}/resources/{folder}/files/{file}")]
pub struct ResourcesUriBuilder<'a> {
    #[param(map_from = "|pb: &PathBuf| pb.to_str().unwrap().to_string()")]
    file: Option<PathBuf>,
    #[param]
    folder: Option<String>,
    #[parent]
    parent: Option<&'a CacheUriLegacyBuilder<'a>>
}

impl ResourcesUriBuilder<'_> {
    /// Produce the resources/{folder}/files URI
    /// endpoint to access read to all files.
    pub fn build_files(&self) -> anyhow::Result<String> {
        if self.file.is_none() && self.folder.is_some() {
            self.build_join("projects")
        } else {
            Err(UriBuildError::Validation.into())
        }
    }
}

impl CacheUriLegacyBuilder<'_> {
    /// Continue the builder into a
    /// `ResourcesUriBuilder`
    pub fn resources(&self) -> ResourcesUriBuilder {
        ResourcesUriBuilder::from_parent(&Rc::new(self))
    }
}

/// Represents the URI endpoints available to
/// manage user favorites.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/favorites/Project/{project_id}", requires = "username_is_none!()")]
pub struct FavoritesUriBuilder<'a> {
    #[param]
    project_id: Option<String>,
    #[parent]
    parent: Option<&'a UsersUriLegacyBuilder<String>>
}

/// Represents the URI endpoints available to
/// manage users associated with a project.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/projects/{project_id}/users", requires = "username_is_none!()")]
#[match_path(path = "{parent}/projects/{project_id}/users/{group_display_name}/{username}", requires = "username_is_none!()")]
pub struct ProjectsUriBuilder<'a> {
    #[param]
    project_id: Option<String>,
    #[param]
    group_display_name: Option<String>,
    #[param]
    username: Option<String>,
    #[parent]
    parent: Option<&'a UsersUriLegacyBuilder<String>>,
}

/// Represents the URI endpoints for user PARs
/// requests (Project Access Request)
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/pars")]
#[match_path(path = "{parent}/pars/{pars_id}")]
#[match_path(path = "{parent}/projects/{project_id}/pars")]
pub struct ProjectAccessUriBuilder<'a> {
    #[param]
    pars_id: Option<String>,
    #[param]
    project_id: Option<String>,
    #[parent]
    parent: Option<&'a ProjectsUriBuilder<'a>>
}

impl ProjectsUriBuilder<'_> {
    /// Continue the builder into a
    /// `ProjectAccessUriBuilder`.
    pub fn pars(&self) -> ProjectAccessUriBuilder {
        ProjectAccessUriBuilder::from_parent(&Rc::new(self))
    }
}

impl UsersUriLegacyBuilder<String> {
    /// Continue the builder into a
    /// `CacheUriBuilder`.
    pub fn cache(&self) -> CacheUriLegacyBuilder {
        CacheUriLegacyBuilder::from_parent(&Rc::new(self))
    }

    /// Continue the builder into a
    /// `FavoritesUriBuilder`.
    pub fn favorites(&self) -> FavoritesUriBuilder {
        FavoritesUriBuilder::from_parent(&Rc::new(self))
    }

    /// Continue the builder into a
    /// `ProjectsUriBuilder`.
    pub fn projects(&self) -> ProjectsUriBuilder {
        ProjectsUriBuilder::from_parent(&Rc::new(self))
    }
}

/// Represents the URI paths available for
/// endpoints meant for managing users.
pub trait UsersUri: Version {
    /// URI endpoints to access user
    /// administration.
    #[inline]
    fn users(&self) -> UserUriBuilder<String> {
        UserUriBuilder::from_parent(self.root_uri().into())
    }
}

/// Represents the URI paths available for
/// endpoints meant for managing users via the
/// legacy API.
pub trait UsersUriLegacy: Version {
    /// URI legacy endpoints to access user
    /// administration.
    #[inline]
    fn users_legacy(&self) -> UsersUriLegacyBuilder<String> {
        UsersUriLegacyBuilder::from_parent(self.root_uri().into())
    }
}
