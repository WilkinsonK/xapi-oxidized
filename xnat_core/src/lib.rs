mod uri;
mod uri_admin;
mod version;

use std::path::Path;

#[cfg(feature = "derive")]
pub use oxinat_derive::*;

pub use crate::uri::{UriBuilder, UriBuildError};
pub use crate::uri_admin::{AdminUri, AdminUriLegacy};
pub use crate::version::Version;

/// Convert a `std::path::Path` into a `String`.
fn pathbuf_to_string(p: &Path) -> String {
    String::from(p.to_str().unwrap())
}
