use std::{fmt::{Debug, Display}, path::PathBuf};

use crate::UriBuilder;


#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/resources")]
#[match_path(path = "{parent}/resources/{resource}")]
#[match_path(path = "{parent}/resources/{resource}/files")]
#[match_path(path = "{parent}/resources/{resource}/files/{file}")]
pub struct ResourcesUriBuilder<'a, Parent>
where
    Parent: Display + Debug,
{
    #[param]
    resource: Option<String>,
    #[param(map_from = "|pb: &PathBuf| pb.to_str().unwrap().to_string()")]
    file: Option<PathBuf>,
    #[parent]
    parent: Option<&'a Parent>
}
