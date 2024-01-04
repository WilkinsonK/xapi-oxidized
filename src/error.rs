use std::error::Error as StdError;
use std::fmt;

use isahc;
use surf;

/// Implements `From<T> for Error` of the
/// specified type.
macro_rules! impl_from_error {
    ($error:ty) => {
        impl From<$error> for Error {
            fn from(value: $error) -> Self {
                Self::from_other(value.into())
            }
        }
    };
}

impl_from_error!(isahc::error::Error);
impl_from_error!(surf::http::Error);
impl_from_error!(surf::http::url::ParseError);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error(pub String);

impl Error {
    pub fn from_str(message: &str) -> Self {
        Self(message.to_string())
    }
    pub fn from_string(message: String) -> Self {
        Self(message.to_owned())
    }
    pub fn from_other(other: Box<dyn StdError>) -> Self {
        Self::from_string(other.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &self.0
    }
}
