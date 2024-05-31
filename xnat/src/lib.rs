pub mod client;
pub mod versions;

#[cfg(feature = "core")]
pub extern crate oxinat_core;
#[cfg(feature = "derive")]
pub extern crate oxinat_derive;

pub use oxinat_core::BuildResult;

pub use client::*;
pub use versions::{V1, V2};
