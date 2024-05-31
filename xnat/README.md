# XAPI Oxidized #
A RESTful abstraction layer library for interacting with XNAT web
APIs. Allows the user to perform actions against a host
programmatically via Rust.

The intent is to be able to build tools and automated tasking.

## Getting started ##
To use this library in your project, you must add it to your
dependencies. The easiest way to do this is by simply doing the
following:

```bash
$ cargo add oxinat --features core
```

It is important to know that the core feature must be utilized as this
unlocks the basic interface. This may be changed in future iterations,
but for now the core stays as a separate feature.

Once installed, you should have access to structs `Xnat` and
`XnatBuilder`, as well as `V1` and `V2`-- the initial version
implemented structs granting access to building URIs mapped from
XAPI documentation.

```rust
use oxinat::Xnat;

let client = Xnat::configure("your.target.host")
    .use_secure(true)
    .with_username("your-username")
    .with_password("your-password")
    .build()
    .expect_err(); // No version implementation set.
```

## XAPI Oxidized API ##

### Versions ###
A `Version` is a trait defined in the `core` feature, granting access
to the root component of **XAPI** **URI**s, and with `URIBuilder`
implemented types and traits, allows you to construct **URI**s in a
progressional manner guided by the `oxinat` internal system.

```rust
use oxinat::{Version, ProjectUri};

fn foo<V: Version>(version: &V) -> Result<(), ()> {
    version.project_data().with_id("PROJECT_ID").build()?;
    Ok(())
}
```

At the above `.build()` call, `oxinat` will try to build a **URI** in
the form of a `Result<String, ..>`. Assuming it is valid, and the
above case should be, the resulting string from unwrapping the the
`Result` should produce something like:

```rust
"{data_uri}/projects/PROJECT_ID"
```

`{data_uri}` should be already pre-formatted. When utilized through
an implementation of `Version`, a call will be made to the type's
`fn data_uri() -> String` method. This will also be true in the case
where `fn root_uri() -> String` is required.

### Custom Versions ###
At the surface level, the units `V1` and `V2` have already been
implemented to access `UriBuilder` implemented types and traits. It
would be recommended to simply use these units, but it is possible
to define your own.

To do this, you will need to install the `derive` feature, but then
after the full API will be unlocked and available.

```toml
[dependencies]
oxinat = { version = "0.6.1", features = ["core", "derive"]}
```

At the time of writing, it would be advisable to instead use the
`full` feature and still make use of the entire suite.

With `derive` enabled, defining a custom `Version` should be
relatively easy with some requirements. You must make a call to
`#[version()]` above the deriving type along with at least declaring
the `root_uri` attribute. This tells the compiler how to construct
the initial `Version` impl, prior to inclusion of **URI** building
traits. Said `#[version()]` declaration comes with a few
additional attributes that are optional.

- `data_uri` defines the value of `fn data_uri() -> String`.
`root_uri` is used instead if omitted.
- `legacy` tells the compiler to only implement sub-traits for legacy
XNAT systems.

An example of deriving from a version would be:

```rust
use oxinat::{Version, ProjectUri, ExperimentUri, SubjectUri};

#[derive(Clone, Version, ProjectUri, ExperimentUri, SubjectUri)]
#[version(root_uri = "xapi", data_uri = "data", legacy = true)]
struct MyVersion;

MyVersion
    .project_data()
    .with_id("SOME_PROJECT")
    .subjects()
    .with_subject("SOME_SUBJECT")
    .experiments()
    .build()?
```

And then the resulting **URI** should look something like this:
```rust
"data/projects/SOME_PROJECT/subjects/SOME_SUBJECT/experiments"
```

### Clients and  Client Builders ###
Another key piece to the puzzle is going to be `Xnat<V>` and
`XnatBuilder<V>`. Where these types allow you to broker calls to
to/from a target XNAT host. As described earlier:

```rust
use oxinat::Xnat;

use crate::MyVersion;

let client = Xnat::configure("your.target.host")
    .use_secure(true)
    .with_username("your-username")
    .with_password("your-password")
    .with_version(MyVersion)
    .build()
    .expect("must build an XNAT client"); // Should produce a client.
```

Where now, we are setting the desired version, with the **URI**
builders we want, we can expect to have a proper `Xnat` client.
