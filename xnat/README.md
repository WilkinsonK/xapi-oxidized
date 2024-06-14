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

### Protocols ###
An effort is being made to predefine some common operations you may
wish to perform. We are defining them as `protocols` where a protocol
is related to come **CRUD** operation against a resource XNAT makes
available to an authorized user.

For clients which implement `ClientCore` and `ClientREST` traits,
as we continue development, these additional traits will be available:

```rust
/// Type is able to implement CREATE requests for
/// a particular model. Upon creation, these
/// methods are expected to then return the
/// individual results, and then a
/// `Ok(Self::Model)` response if the request is
/// successful.
trait Create<M> {
    /// Attempt to send a CREATE request to the
    /// XNAT server for **multiple** models.
    fn create_many(&self, models: M) -> Vec<PinnedFuture<'_, M>>;
    /// Attempt to send a CREATE request to the
    /// XNAT server for **one** model.
    fn create_once(&self, model: M) -> anyhow::Result<M>;
}

/// Type is able to implement RETRIEVE requests
/// for a particular model.
trait Retrieve<M> {
    /// Get all instances of a particular model
    /// available to the user via the XNAT host.
    async fn get_all(&self) -> anyhow::Result<Vec<M>>;
    /// Get all instances of a particular model
    /// using another model as the query
    /// parameters for the request.
    async fn get_any_from(&self, model: &M) -> anyhow::Result<Vec<M>>;
    /// Get one instance of a particular model
    /// using another model as the query
    /// parameters for the request.
    async fn get_one_from(&self, model: &M) -> anyhow::Result<M>;
}

/// Type is able to implement UPDATE or UPSERT
/// requests for a particular model.
trait Update<M> {
    /// Attempt to send **multiple** UPDATE
    /// requests to the XNAT host.
    fn update_many(&self, models: M) -> Vec<PinnedFuture<'_, M>>;
    /// Attempt to send **one** UPDATE request to
    /// the XNAT host.
    async fn update_once(&self, model: M) -> anyhow::Result<M>;
}

/// Type is able to implement DELETE requests for
/// a particular model.
trait Delete<M> {
    /// Attempt to send **multiple** DELETE
    /// requests to the XNAT host.
    fn delete_many(&self, models: M) -> Vec<PinnedFuture<'_, M>>;
    /// Attempt to send **one** DELETE request to
    /// the XNAT host.
    async fn delete_once(&self, model: M) -> anyhow::Result<M>;
}
```

#### Retrieve ####
The `Retrieve` trait has already be implemented on a generic **Xnat**
client for certain models.

- `Project`
- `Subject`
- `Experiment`
- `Scan`
- `Assessor`
- `Resource`
- `Plugin`

The `Retrieve` trait itself is meant to allow you to **GET** resources
from your target XNAT instance.

```rust
use oxinat::{ClientCore, ClientToken, Xnat}
use oxinat::models::Project;
use oxinat::protocols::Retrieve;

use crate::{MyVersion, client};

// Should retrieve all project records from your
// XNAT instance.
let found: Vec<Project> = client.get_all().await.unwrap();

// Should retrieve one project which matches the
// project ID.
let mut project = Project::default();
project.id.clone_from(&Some("SOME_PROJECT_ID".into()));
let found = client.get_one_from(&project).await.unwrap();
```

The predefined getters, when performing a query, tries to construct
the request path by first extracting relevant identifiers and
consuming the remaining populated fields as query parameters.
