//! Defines core behaviors for a OXR `Session` struct. We use this
//! implementation to create a session that can be used as the REST
//! broker for building requests while managing authentication and
//! client configuration.

use crate::core::error::Error;

use std::env;
use std::fs;
use std::io::Write;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use futures::{Future, FutureExt};
use serde::Serialize;
use surf;
use surf::http::auth::BasicAuth;

/// Defines the most common pattern shared between
/// methods defined in the `SessionREST` trait.
macro_rules! init_build_request {
    ($parent: expr, $method:ident) => {
        match $parent.client() {
            Ok(client) => {
                let path = $parent.request_path.join("/");
                $parent.request_path.clear();
                $parent.request_builder = Some(client.$method(path));
                Ok($parent)
            },
            Err(e) => Err(e)
        }
    };
}

/// Using a map closure, converts a `PathBuf`
/// result or option into another thing.
macro_rules! map_pathbuf {
    ($path_buf:ident, $map:expr) => {
        $path_buf.as_ref().map($map).unwrap_or_default()
    };
}

/// Initial search paths provided to find a .netrc
/// configuration. Paths will be searched in order
/// from 0th to last.
///
/// The keywords used below, **this, last, home &
/// name**, represent path components that are
/// replaced prior to actual search for the paths
/// they represent.
const NETRC_SEARCH_PATHS: [&str; 3] = [
    "{this}/{name}",
    "{last}/{name}",
    "{home}/{name}",
];
/// Connect/read timeout for RESTful operatations
/// against the host machine. This value is used
/// as the default timeout.
const REST_TIMEOUT: Duration = Duration::from_secs(15);

/// Wrapper around `std::result::Result` to encase
/// our own Error type with a result.
pub type Result<T> = std::result::Result<T, Error>;

/// Creates the formatting function which takes a
/// .netrc search path pattern and returns a more
/// realized path to look through.
fn netrc_path_formatter<'a>(name: &'a str) -> impl FnMut(&str) -> String + 'a {
    |path: &str| {
        let buf = home::home_dir();
        let home = map_pathbuf!(buf, |h| h.to_str().unwrap());

        let buf = env::current_dir();
        let this = map_pathbuf!(buf, |th| th.to_str().unwrap());
        let last = map_pathbuf!(buf, |lt| lt.parent().unwrap().to_str().unwrap());

        path
            .replace("{home}", home)
            .replace("{this}", this)
            .replace("{last}", last)
            .replace("{name}", name)
    }
}

fn netrc_find_machine<'a: 'b, 'b>(name: &'a str, hostname: &'b str) -> Result<netrc_rs::Machine> {
    let host = surf::Url::parse(hostname);
    if !host.is_ok() {
        return Err(host.unwrap_err().into());
    }
    let host = host.unwrap().host_str().unwrap().to_string();

    // Search for valid .netrc file and try to
    // extract a matching machine to the host
    // given in this method call.
    let mut formatter = netrc_path_formatter(name);
    for p in NETRC_SEARCH_PATHS {
        let path = formatter(p);
        if !fs::metadata(&path).is_ok() {
            continue;
        }
        let data = fs::read_to_string(path);
        if !data.is_ok() {
            continue;
        } else if data.as_ref().unwrap().len() < 1 {
            continue;
        }
        let data = netrc_rs::Netrc::parse(data.unwrap(), false);
        if !data.is_ok() {
            continue;
        }
        let mach = data
            .as_ref()
            .unwrap()
            .machines
            .iter()
            .find(|p| {
                let left  = p.name.as_ref().unwrap();
                *left == host
            });
        if mach.is_some() {
            return Ok(mach.unwrap().to_owned())
        }
        }
    Err(Error::from_string(format!("failed to find config for {}", hostname)))
}

/// Implements methods to instantiate a new
/// `Session` instance.
pub trait NewSession<'a, 'b, 'c> {
    /// Create a new `Session` from basic
    /// credentials.
    fn new(host: &'a str, username: &'b str, password: &'c str) -> Self;
    /// Create a new `Session` from a machine
    /// found in the available .netrc files.
    fn from_host(hostname: &'a str) -> Self;
}

/// Implements the core methods for a session
/// struct.
pub trait SessionCore {
    /// Try to get the configured client.
    fn client(self) -> Result<surf::Client>;
    /// Configure the inner client object.
    fn configure(&mut self) -> Result<()>;
    /// Get the URL representation of the
    /// hostname.
    fn url(&self) -> Result<surf::Url>;
}

/// Implements methods to modify session values in
/// place.
pub trait SessionMut<'a> {
    /// Set the target host name.
    fn set_hostname(self, value: &'a str) -> Self;
    /// Set the client password.
    fn set_password(self, value: &'a str) -> Self;
    /// Set the client username.
    fn set_username(self, value: &'a str) -> Self;
}

/// Implements methods to use the REST calls as
/// the inner client.
pub trait SessionREST<'a>
where
    Self: Sized,
{
    // Not implementing 'head' as XAPI does not
    // support this method.
    /// Create an initial DELETE request.
    fn delete(&mut self) -> Result<Self>;
    /// Create an initial GET request.
    fn get(&mut self) -> Result<Self>;
    /// Create an initial PATCH request.
    fn patch(&mut self) -> Result<Self>;
    /// Create an initial POST request.
    fn post(&mut self) -> Result<Self>;
    /// Create an initial PUT request.
    fn put(&mut self) -> Result<Self>;
}

/// Implements methods to apply Query parameters
/// to a request.
pub trait SessionQuery<'a>
where
    Self: Sized,
{
    /// Add a query argument to the request URI.
    fn with_arg(self, uri: &'a str) -> Self;
    /// Append a query option to the request.
    fn with_opt<T: Serialize>(self, opt: &T) -> Result<Self>;
    /// Add a URI component to the path.
    fn with_uri(self, uri: &'a str) -> Self;
}

/// Session representing the REST client.
pub struct Session{
    client:   Option<surf::Client>,
    auth:     BasicAuth,
    hostname: String,
    /// state holder for whenever a session is
    /// in the middle of building a request.
    request_builder: Option<surf::RequestBuilder>,
    request_path:    Vec<String>,
}

impl Session {
    fn client(self) -> Result<surf::Client> {
        match self.client {
            Some(client) => Ok(client),
            None => Err(Error::from_str("session client not configured"))
        }
    }
    fn configure(&mut self) -> Result<()> {
        let mut config = surf::Config::new();
        config = config.set_base_url(self.url()?);
        config = config.set_timeout(Some(REST_TIMEOUT));
        config = config.add_header(self.auth.name(), self.auth.value())?;

        match config.try_into() {
            Ok(client) => {
                self.client = Some(client);
                Ok(())
            },
            Err(err) => Err(Error::from_other(err.into()))
        }
    }
    fn url(&self) -> Result<surf::Url> {
        surf::Url::parse(&self.hostname).map_err(|err| err.into())
    }
}

impl Clone for Session {
    fn clone(&self) -> Self {
        Self::new(&self.hostname, self.auth.username(), self.auth.password())
    }
}

impl Future for Session {
    type Output = Result<surf::Response>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let ret = self.request_builder
            .as_mut()
            .unwrap()
            .poll_unpin(cx)
            .map_err(|err| err.into());
        self.request_builder = None;

        ret
    }
}

impl<'a, 'b, 'c> NewSession<'a, 'b, 'c> for Session {
    fn new(hostname: &'a str, username: &'b str, password: &'c str) -> Self {
        let mut session = Self{
            client: None,
            auth: BasicAuth::new(username, password),
            hostname: hostname.to_owned(),
            request_builder: None,
            request_path: vec![],
        };
        if let Err(e) = session.configure() {
            let mut stream = std::io::stderr();
            let _ = writeln!(stream, "warn: could not configure session: {e}");
        }
        session
    }
    fn from_host(hostname: &'a str) -> Self {
        match netrc_find_machine(".netrc", hostname) {
            Ok(mach) => {
                let digest = |a: Option<String>| {
                    a.to_owned().unwrap_or("".to_string())
                };
                let user = digest(mach.login);
                let pass = digest(mach.password);
                Self::new(hostname, user.as_str(), pass.as_str())
            },
            Err(e) => {
                let mut stream = std::io::stderr();
                let _ = writeln!(stream, "warn: could not create session from host: {e}");
                Self::new(hostname, "", "")
            }
        }
    }
}

impl<'a> SessionMut<'a> for Session {
    fn set_hostname(mut self, value: &'a str) -> Self {
        self.hostname = value.to_owned();
        self
    }
    fn set_password(mut self, value: &'a str) -> Self {
        self.auth = BasicAuth::new(self.auth.username(), value);
        self
    }
    fn set_username(mut self, value: &'a str) -> Self {
        self.auth = BasicAuth::new(value, self.auth.password());
        self
    }
}

impl<'a> SessionREST<'a> for Session {
    fn delete(&mut self) -> Result<Self> {
        init_build_request!(self.clone(), delete)
    }
    fn get(&mut self) -> Result<Self> {
        init_build_request!(self.clone(), get)
    }
    fn patch(&mut self) -> Result<Self> {
        init_build_request!(self.clone(), patch)
    }
    fn post(&mut self) -> Result<Self> {
        init_build_request!(self.clone(), post)
    }
    fn put(&mut self) -> Result<Self> {
        init_build_request!(self.clone(), put)
    }
}

impl<'a> SessionQuery<'a> for Session {
    fn with_arg(mut self, uri: &'a str) -> Self {
        self.request_path.push(uri.to_string());
        self
    }
    fn with_opt<T: Serialize>(mut self, opt: &T) -> Result<Self> {
        match self.request_builder {
            Some(builder) => {
                // Only return `Self` if the
                // builder can add the option
                // successfully.
                match builder.query(opt) {
                    Ok(builder) => {
                        self.request_builder = Some(builder);
                        Ok(self)
                    },
                    Err(e) => Err(e.into())
                }
            },
            None => Err(Error::from_str("builder not configured"))
        }
    }
    fn with_uri(mut self, uri: &'a str) -> Self {
        self.request_path.push(uri.to_string());
        self
    }
}
