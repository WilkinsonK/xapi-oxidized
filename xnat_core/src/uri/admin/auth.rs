use std::{fmt::Debug, sync::Arc};

use oxinat_derive::uri_builder_alias;

use crate::{UriBuilder, Version};

uri_builder_alias!(AuthenticateUriBuilder);
ImplAuthenticateUriBuilder! {
    (String),
}

/// Represent the URI paths responsible for user
/// authentication.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}")]
pub struct AuthUriLegacyBuilder<Parent>
where
    Parent: AuthenticateUriBuilder
{
    #[parent]
    parent: Option<Arc<Parent>>
}

/// Represents the URI paths responsible for
/// specific actions meant for user authentication.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/services")]
pub struct ServicesUriBuilder<'a> {
    #[parent]
    parent: Option<&'a AuthUriLegacyBuilder<String>>
}

/// Represents the URI paths available for user
/// token management.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/tokens")]
pub struct TokensUriBuilder<'a> {
    #[parent]
    parent: Option<&'a ServicesUriBuilder<'a>>
}

/// Represents the URI paths available to issue
/// a user authentication.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/issue")]
#[match_path(path = "{parent}/issue/user/{username}")]
pub struct IssueUserUriBuilder<'a> {
    #[param]
    username: Option<String>,
    #[parent]
    parent: Option<&'a TokensUriBuilder<'a>>
}

/// Represents the URI paths available to validate
/// user tokens.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/validate/{token}/{secret}")]
pub struct ValidateUserUriBuilder<'a> {
    #[param]
    secret: Option<String>,
    #[param]
    token: Option<String>,
    #[parent]
    parent: Option<&'a TokensUriBuilder<'a>>
}

/// Represents the URI paths available to
/// invalidate user tokens.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/invalidate/{token}/{secret}")]
pub struct InValidateUserUriBuilder<'a> {
    #[param]
    secret: Option<String>,
    #[param]
    token: Option<String>,
    #[parent]
    parent: Option<&'a TokensUriBuilder<'a>>
}

impl TokensUriBuilder<'_> {
    /// Continue the builder into a
    /// `ValidateUserUriBuilder`
    pub fn invalidate(&self) -> InValidateUserUriBuilder {
        InValidateUserUriBuilder::from_parent(self)
    }

    /// Continue the builder into a
    /// `IssueUserUriBuilder`.
    pub fn issue(&self) -> IssueUserUriBuilder {
        IssueUserUriBuilder::from_parent(self)
    }

    /// Continue the builder into a
    /// `ValidateUserUriBuilder`
    pub fn validate(&self) -> ValidateUserUriBuilder {
        ValidateUserUriBuilder::from_parent(self)
    }
}

impl ServicesUriBuilder<'_> {
    /// Produces the endpoint used to request a
    /// new user session.
    pub fn build_auth(&self) -> anyhow::Result<String> {
        self.build_join("auth")
    }

    /// Continue the builder into a
    /// `TokensUriBuilder`.
    pub fn tokens(&self) -> TokensUriBuilder {
        TokensUriBuilder::from_parent(self)
    }
}

impl AuthUriLegacyBuilder<String> {
    /// Produces the endpoint used for managing
    /// the logged in user's session.
    pub fn build_jsessionid(&self) -> anyhow::Result<String> {
        self.build_join("JSESSIONID")
    }

    /// Continue the builder into a
    /// `AuthServicesUriBuilder`.
    pub fn services(&self) -> ServicesUriBuilder {
        ServicesUriBuilder::from_parent(&Arc::new(self))
    }
}

/// Represents the URI paths available for user
/// authentication.
pub trait AuthUriLegacy: Version {
    fn auth_legacy(&self) -> AuthUriLegacyBuilder<String> {
        AuthUriLegacyBuilder::from_parent(self.data_uri().into())
    }
}
