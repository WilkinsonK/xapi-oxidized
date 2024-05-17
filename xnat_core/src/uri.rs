use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UriBuildError {
    #[error("pattern could not be determined from arguments")]
    UnrecognizedPattern,
}

/// Type is able to construct a URI usable for
/// making REST calls.
pub trait UriBuilder: Display {
    /// Build the resulting URI from this builder.
    fn build(&self) -> anyhow::Result<String>;
}

impl UriBuilder for String {
    fn build(&self) -> anyhow::Result<String> {
        Ok(self.to_owned())
    }
}
