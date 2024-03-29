//! XAPI endpoint bindings. Includes request building, such as
//! modeling URIs and applying options available to those endpoints.
mod broker;
pub use broker::{Broker, BrokerAttributes, BrokerVersion, Latest, Legacy};
mod projects;

