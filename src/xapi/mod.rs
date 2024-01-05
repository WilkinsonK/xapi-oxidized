//! XAPI endpoint bindings. Includes request building, such as
//! modeling URIs and applying options available to those endpoints.
mod broker;
pub use broker::{Broker, BrokerAttributes, BrokerVersion, Latest, Legacy};

mod projects;

/// Represents a status type potentially returned
/// by the XNAT host.
pub struct Status(u16, &'static str);
pub const SUCCESS:        Status = Status(200, "request was successful");
pub const CREATED:        Status = Status(201, "resource created Successfully");
pub const INVALID_DATA:   Status = Status(400, "data submitted was invalid");
pub const NOT_AUTHORIZED: Status = Status(401, "user is not granted access");
pub const NOT_FOUND:      Status = Status(404, "resource was not found");
pub const ALREADY_EXISTS: Status = Status(409, "resource already exists");
pub const HOST_FAULT:     Status = Status(500, "host XNAT encountered an error");

impl Status {
    /// The status code.
    pub fn code(self) -> u16 { self.0 }
    /// The response description.
    pub fn description(self) -> &'static str { self.1 }
}

impl From<u16> for Status {
    fn from(value: u16) -> Self {
        match value {
            200 => SUCCESS,
            201 => CREATED,
            400 => INVALID_DATA,
            401 => NOT_AUTHORIZED,
            404 => NOT_FOUND,
            409 => ALREADY_EXISTS,
            500 => HOST_FAULT,
            _ => panic!("unsupported status code {value}")
        }
    }
}
