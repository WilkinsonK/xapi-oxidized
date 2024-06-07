use reqwest::Method;
use thiserror::Error;

/// Errors specific to the purpose of interactions
/// between an XNAT client and the host.
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("could not successfully acquire auth token ({0})")]
    AuthFailure(u16),
    #[error("coult not successfully release auth token ({0})")]
    DeauthFailure(u16),
    #[error("error occured attempting a transaction with the host ({0})")]
    ServerFailure(u16),
    #[error("`{1}` does not support method `{0}`")]
    UnsupportedMethod(Method, String)
}
