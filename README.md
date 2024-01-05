# XAPI Oxidized #
Interacts with a remote XNAT via REST exposing the **XAPI** as
bindings in Rust.

```rust
use xapi_oxidized::{self as xapi, NewSession, SessionREST};

// Pass a base URL as the host name in this
// method call.
let mut session = xapi::Session::from_host("");
// Build and send a new request using `surf`
// API.
let mut req = session.get()?.await?
// Attempt to parse the response body.
println!("from body: [{}]({})", req.body_string().await?, req.status());
```

## Construct a Request ##
Requests are constructed using `surf::RequestBuilder` under-the-hood.
On the surface, a `Session` object can interact with the builder to
add path arguments and query parameters.

```rust
use xapi_oxidized::{self as xapi, NewSession, SessionREST, SessionQuery};

#[derive(Serialize, Deserialize)]
struct Index {
    page: u32,
}

// Pass a base URL as the host name in this
// method call. `::from_host` will try to find a
// `.netrc` file that has a definition for the
// host and use that machine to configure the
// session.
let mut session = xapi::Session::from_host("https://phake-digital-library.org");

// Path components are added to a `Vector`. Used
// as a buffer, the components are composed to
// initialize the `surf::RequestBuilder`.
//
// NOTE: The builder is managed internally by the
// session. Using any of the methods from the
// `SessionREST` trait will compile the path and
// clear the buffer.
session = session
    .with_uri("books")
    .with_arg(format!("author/{}", "stephen-king"))
    .with_arg(format!("title/{}", "thinner"))
    .get()?;

// Options must be applied to the internal builder
// after it has been created.
// Once the session has been awaited, the builder
// will be cleared and the respose returned to the
// user.
let mut res = session.with_opt(Index{page: 72})?.await?;
println!("from body: [{}]({})", req.body_string().await?, req.status());
```

## Under the Hood ##
**xapi_oxidized** uses a number of dependencies. See the below as reference to
the projects used in this one. While there is a collection of implicit deps,
the listed items are only those we use explicitly.

- [home](https://github.com/rust-lang/cargo/tree/master/crates/home) canonical definitions of system and rust **home** directory paths
- [netrc-rs](https://github.com/yjhmelody/netrc-rs) .netrc file parser
- [surf](https://github.com/http-rs/surf) HTTP backend client
