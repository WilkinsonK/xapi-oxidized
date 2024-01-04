# Oxidized XNAT REST
Interacts with a remote XNAT via REST exposing the **XAPI** as
bindings in Rust.

```rust
    use std::error::Error;
    use oxidized_xnat_rest::{self as oxr, NewSession, SessionREST};

    // Pass a base URL as the host name in this
    // method call.
    let session = oxr::Session::from_host("");

    // Build and send a new request using `surf`
    // API.
    let req = session
        .get("xapi/users/username")
        .unwrap()
        .await;

    // Attempt to parse the response body.
    if let Ok(mut res) = req {
        println!("from body: [{}]({})", res.body_string().await?, res.status());
        Ok(())
    } else {
        Err(req.unwrap_err().into())
    }
```

## Under the Hood
OXR uses a number of dependencies. See the below as reference to the
projects used in this one.

- [surf](https://github.com/http-rs/surf) HTTP backend client
- [netrc-rs](https://github.com/yjhmelody/netrc-rs) .netrc file parser
