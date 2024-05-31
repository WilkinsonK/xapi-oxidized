use reqwest::Method;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("could not successfully acquire auth token ({0})")]
    AuthFailure(u16),
    #[error("coult not successfully release auth token ({0})")]
    DeauthFailure(u16),
    #[error("error occured attempting a transaction with the host ({0})")]
    ServerFailure(u16),
    #[error("`{0}` does not support method `{1}`")]
    UnsupportedMethod(Method, String)
}
