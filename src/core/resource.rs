//! Defines general traits for creating, locating & modifying data
//! from the REST API.
use crate::core::Status;
use crate::core::session::{QueryArgs, QueryOpts, Result};
use serde::Serialize;

/// Representative of a generic resource
pub struct Resource<R>(pub R);
pub type Resources<R> = Vec<Resource<R>>;

/// Implementation for locating a resource via
/// RESTful operation.
pub trait GetResource<A: ToString, O: Serialize> {
    /// Performs a REST call to try and find
    /// resources.
    fn locate<R>(self, args: QueryArgs<A>, options: QueryOpts<O>) -> Result<(Resources<R>, Status)>;
}
/// Implementation for modifying a resource
/// in-place via RESTful operation.
pub trait ModResource<A: ToString, O: Serialize> {
    /// Performs a REST call to try and change
    /// resource data.
    fn modify<R>(self, args: QueryArgs<A>, options: QueryOpts<O>) -> Result<Status>;
}
/// Implementation for creating a resource via
/// RESTful operation.
pub trait NewResource<A: ToString, O: Serialize> {
    /// Performs a REST call to try and create
    /// a new resource.
    fn create(self, args: QueryArgs<A>, options: QueryOpts<O>) -> Result<Status>;
}
