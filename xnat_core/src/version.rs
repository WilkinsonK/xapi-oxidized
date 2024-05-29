use std::fmt::Display;

/// Represents the URI paths available for a
/// specific version of the XNAT REST API.
pub trait Version: Display {
    /// Represents the root URI used by the API
    /// version.
    fn root_uri(&self) -> String;
    /// Represents the root URI used by the API
    /// to access legacy  and data endpoints.
    fn data_uri(&self) -> String;
}
