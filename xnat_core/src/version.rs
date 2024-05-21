/// Represents the URI paths available for a
/// specific version of the XNAT REST API.
pub trait Version {
    /// Represents the root URI used by the API
    /// version.
    fn root_uri(&self) -> String;
}
