use reqwest::Method;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("`{0}` does not support method `{1}`")]
    UnsupportedMethod(Method, String)
}
