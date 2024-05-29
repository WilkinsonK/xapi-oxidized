use attribute_derive::FromAttr;
use thiserror::Error;

use crate::Attributes;

/// Errors specific to `Version` derivative
/// actions.
#[derive(Debug, Error)]
enum VersionAttrsError {
    #[error("`root_uri` was not declared")]
    RootURINotDeclared
}

/// Represents attributes passed to `Version`
/// derived implementation.
#[derive(FromAttr, Debug)]
#[attribute(ident = version)]
struct VersionAttrs {
    legacy:   Option<bool>,
    root_uri: Option<String>,
    data_uri: Option<String>,
}

/// Attempt to parse out the attribute `root_uri`
/// to be used in `Version` derived
/// implementation.
pub fn derive_version_parse_root_uri(attrs: &Attributes) -> anyhow::Result<String> {
    VersionAttrs::from_attributes(attrs)?
        .root_uri
        .ok_or(VersionAttrsError::RootURINotDeclared.into())
}

/// Attempt to parse out the attribute
/// `root_uri_legacy` to be used in `Version`
/// derived implementation.
pub fn derive_version_parse_data_uri(attrs: &Attributes) -> anyhow::Result<String> {
    let va = VersionAttrs::from_attributes(attrs)?;
    va
        .data_uri
        .or(va.root_uri)
        .ok_or(VersionAttrsError::RootURINotDeclared.into())
}

/// Attempt to parse out the attribute `legacy`
/// to be used to determine if legacy traits
/// should be implemented.
pub fn derive_version_parse_legacy(attrs: &Attributes) -> bool {
    VersionAttrs::from_attributes(attrs)
        .unwrap()
        .legacy
        .unwrap_or_default()
}
