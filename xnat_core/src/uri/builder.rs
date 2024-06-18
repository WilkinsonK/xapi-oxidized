use std::fmt::Display;

use thiserror::Error;

pub type BuildResult = anyhow::Result<String>;

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
    fn build(&self) -> BuildResult;
    /// Build the resulting URI with an additional
    /// component appended at the end.
    #[inline]
    fn build_join<UB: UriBuilder>(&self, other: UB) -> BuildResult {
        Ok([self.build()?, other.build()?].join("/"))
    }
    /// Build the resulting URI with an additional
    /// component appended at the end, otherwise
    /// returns an error if the predicate isn't
    /// met.
    #[inline]
    fn build_join_if<UB: UriBuilder>(&self, other: UB, predicate: fn(&Self) -> bool) -> BuildResult {
        predicate(self)
            .then(|| self.build_join(other))
            .or_else(|| Err(UriBuildError::UnrecognizedPattern.into()).into())
            .unwrap()
    }
}

impl UriBuilder for str {
    fn build(&self) -> crate::BuildResult {
        Ok(self.to_string())
    }
}

impl<T> UriBuilder for T
where
    T: Display + AsRef<str>
{
    fn build(&self) -> BuildResult {
        Ok(self.as_ref().to_owned())
    }
}
