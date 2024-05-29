use std::fmt::Display;

use crate::UriBuilder;

#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/projects")]
#[match_path(path = "{parent}/projects/{shared}")]
pub struct SharedProjectUriBuilder<'a, Parent>
where
    Parent: Display,
{
    #[param]
    shared: Option<String>,
    #[parent]
    parent: Option<&'a Parent>
}
