use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UriBuildError {
    #[error("pattern could not be determined from arguments")]
    UnrecognizedPattern,
    #[error("attempted to build an invalid URI path")]
    Validation
}

/// Type is able to construct a URI usable for
/// making REST calls.
pub trait UriBuilder: Display {
    /// Build the resulting URI from this builder.
    fn build(&self) -> anyhow::Result<String>;
    /// Build the resulting URI with an additional
    /// component appended at the end.
    #[inline]
    fn build_join<UB: UriBuilder>(&self, other: UB) -> anyhow::Result<String> {
        Ok([self.build()?, other.build()?].join("/"))
    }
}

impl UriBuilder for String {
    fn build(&self) -> anyhow::Result<String> {
        Ok(self.to_owned())
    }
}

impl UriBuilder for &str {
    fn build(&self) -> anyhow::Result<String> {
        Ok(self.to_string())
    }
}
