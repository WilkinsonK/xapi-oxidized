# XAPI Oxidized #
![Crates.io Version](https://img.shields.io/crates/v/oxinat?style=for-the-badge)
![GitHub License](https://img.shields.io/github/license/WilkinsonK/xapi-oxidized?style=for-the-badge)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/WilkinsonK/xapi-oxidized/test.yml?style=for-the-badge)

An encrusted interface for managing and maintaining an XNAT instance,
via REST, mapping XNAT's API to a usable Rust client.

### NOTICE ###
This project is still in alpha and it's internal API is very unstable.
At the time of writing this, it has already gone over 3 rewrites due
to issues including, but not limited to, poor understanding of Rust
and challenges with the overflexibility of the REST API.

## Target Goals ##
This project ultimately should provide a user of this library with the
tools necessary to easily authenticate and interact with an XNAT web
server, via the XNAT standard interface. With that, there are some
items that need to be completed before this project can be considered
**production ready**:

- [x] Well defined interface for building the URI endpoints.
- [x] A simple to use, and extensible object which represents the
      client.

### Additonal Goals ###
These goals are more nice-to-haves, but will open up development and
maintenance of the project to broader use cases, making it also more
useful to a wider audience.

- [x] Expose portions of the core and derivitive sub-crates to allow
      for users to implement access to REST calls outside of the XNAT
      standard interface.
- [x] Optional support for logging specific parts of our interface for
      better debugging.
- [ ] Optional support for building this project as a `Python` module.

## Current status ##
`oxinat` is to a point of being relatively complete for the purposes
of being useful in other projects. Moving forward, progress will be
tracked using the [XAPI Oxidized](https://github.com/users/WilkinsonK/projects/3)
project board to track issues and development.

## Using this Project ##
While this project is still in alpha, there is a lot to be desired
from it in order to be a fully fledged library crate. However, listed
here will be some of the principle traits and objects a user can
expect to work with.

### The Version ###
We want this library to be compatible with the the latest version of
the XNAT API, but also compatible with legacy and potentially future
versions. There are already two implementations that should support
this defined in the `oxinat` crate:

```rust
use oxniat::{V1, V2};
```

A `Version` is a trait implemented at compile time by its
corresponding derive macro and it is required to implement-- or derive
from-- subsequent traits such as `oxinat_core::AdminUri` or
`oxinat_core::SystemUri`. `V1` in this case currently represents the
builder methods available for a legacy XNAT API and the latter, `V2`
the latest version of that same interface.

You can also define your own, and it is recommended to implement the
mentioned traits from their derivitive functions:

```rust
use oxinat::oxinat_core::{Version, AdminUri, AdminUriLegacy};

#[derive(Version, AdminUri)]
struct MyVersion;

#[derive(Version, AdminUri)]
#[version(root_uri = "custom_root", legacy = true)]
struct MyCustomVersion;
```

#### NOTICE ####
You will need to have either included the `core` feature from the
`oxinat` library, or add `oxinat_core` directly to your project and
then import those traits respectively.
