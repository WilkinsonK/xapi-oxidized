# Oxidized XNAT REST
Interacts with a remote XNAT via REST exposing the **XAPI** as
bindings in Rust.

```rust
    use oxidized_xnat_rest::{self as oxr, NewSession, SessionREST};

    // Pass a base URL as the host name in this
    // method call.
    let session = oxr::Session::from_host("");
    // Build and send a new request using `surf`
    // API.
    let mut req = session.get("xapi/users/username")?.await?
    // Attempt to parse the response body.
    println!("from body: [{}]({})", req.body_string().await?, req.status());
```

## Under the Hood
OXR uses a number of dependencies. See the below as reference to the
projects used in this one.

- [surf](https://github.com/http-rs/surf) HTTP backend client
- [netrc-rs](https://github.com/yjhmelody/netrc-rs) .netrc file parser
