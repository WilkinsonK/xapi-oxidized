use std::{fmt::Debug, rc::Rc};

use oxinat_derive::uri_builder_alias;

use crate::{UriBuilder, Version};

uri_builder_alias!(UsersUriBuilder);
ImplUsersUriBuilder! {
    (String),
}
ImplUsersUriBuilder! {
    (GroupsUriBuilder<Parent>, Parent),
    (UserUriBuilder<Parent>, Parent),
}

/// Validator for ensuring that a
/// `UserUriBuilder`, as a parent, does have the
/// parameter `username` set.
macro_rules! username_is_present {
    () => {
        |ub: &Self| ub.parent.as_ref().is_some_and(|p| p.username.is_some())
    };
    ($builder:ident) => {
        $builder.parent.as_ref().is_some_and(|p| p.username.is_some())
    };
}

#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/groups")]
pub struct GroupsUriBuilder<Parent>
where
    Parent: UsersUriBuilder,
{
    #[parent(requires="username_is_present!()")]
    parent: Option<Rc<UserUriBuilder<Parent>>>
}

#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/users")]
#[match_path(path = "{parent}/users/{username}")]
pub struct UserUriBuilder<Parent>
where
    Parent: UsersUriBuilder,
{
    #[param]
    username: Option<String>,
    #[parent]
    parent: Option<Rc<Parent>>
}

/// Represents the URI paths available for
/// endpoints meant for managing users.
pub trait UsersUri: Version {
    #[inline]
    fn users(&self) -> UserUriBuilder<String> {
        UserUriBuilder::from_parent(self.root_uri().into())
    }
}
