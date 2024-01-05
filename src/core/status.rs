//! Defines constant values that represent status codes supported by
//! this project.
use std::error::Error;
use std::fmt;

use surf;

/// Return code from a REST API.
pub type StatusCode = usize;
/// Represents a status type potentially returned
/// by the XNAT host.
#[derive(Debug)]
pub struct Status(StatusCode, &'static str);

impl Status {
    /// The status code.
    pub fn code(&self) -> StatusCode { self.0 }
    /// The response description.
    pub fn desc(&self) -> &'static str { self.1 }
    /// Shorthand for whether the status code
    /// returned is either a host or user error.
    pub fn is_failure(&self) -> bool {
        self.is_host_error() || self.is_user_error()
    }
    /// Whether status code is an error caused by
    /// the host.
    pub fn is_host_error(&self) -> bool {
        (500..600).contains(&self.code())
    }
    /// Whether status code returned indicates the
    /// server accepted the request.
    pub fn is_success(&self) -> bool {
        (200..400).contains(&self.code())
    }
    /// Whether status code is an error caused by
    /// improper use from the user.
    pub fn is_user_error(&self) -> bool {
        (400..500).contains(&self.code())
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.desc(), self.code())
    }
}

impl Error for Status {
    fn description(&self) -> &str {
        self.desc()
    }
}

impl From<surf::Response> for Status {
    fn from(value: surf::Response) -> Self {
        Self::from(value.status() as StatusCode)
    }
}

/// Expands its entries to create interchangable
/// constants between `Status` and `StatusCode`.
macro_rules! status_interop_txl {
    ($($name: ident => ($code:literal, $desc:literal),)+) => {
        $(pub const $name: Status = Status($code, $desc);)+

        impl From<StatusCode> for Status {
            fn from(value: StatusCode) -> Self {
                match value {
                    $($code => $name,)+
                    _ => panic!("unsupported status code {value}")
                }
            }
        }

        impl From<Status> for StatusCode {
            fn from(value: Status) -> Self {
                match value.0 {
                    $($code => $code,)+
                    _ => panic!("unsupported status code {}", value.0)
                }
            }
        }
    };
}

status_interop_txl! {
    SUCCESS => (200, "request was successful"),
    CREATED => (201, "resource created successfully"),
    INVALID_DATA => (400, "data submitted was invalid"),
    NOT_AUTHORIZED => (401, "user was not granted access"),
    NOT_FOUND => (404, "resource was not found"),
    ALREADY_EXISTS => (409, "resource already exists"),
    HOST_FAULT => (500, "host encountered an error."),
}
