//! Defines core behaviors for a OXR `Session` struct. We use this
//! implementation to create a session that can be used as the REST
//! broker for building requests while managing authentication and
//! client configuration.

use crate::error::Error;

use std::env;
use std::fs;
use std::io::Write;
use std::time::Duration;

use surf;
use surf::http::auth::BasicAuth;

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
    |p: &str| {
        let home = home::home_dir();
        let home = home
            .as_ref()
            .map(|h| h.to_str().unwrap())
            .unwrap_or_default();

        let this = env::current_dir();
        let this = this
            .as_ref()
            .map(|th| th.to_str().unwrap())
            .unwrap_or_default();

        let last = env::current_dir();
        let last = last
            .as_ref()
            .map(|lt| lt.parent().unwrap().to_str().unwrap())
            .unwrap_or_default();

        p
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
pub trait SessionREST<'a> {
    /// Create an initial DELETE request.
    fn delete(self, uri: &'a str) -> Result<surf::RequestBuilder>;
    /// Create an initial GET request.
    fn get(self, uri: &'a str) -> Result<surf::RequestBuilder>;
    /// Create an initial PATCH request.
    fn patch(self, uri: &'a str) -> Result<surf::RequestBuilder>;
    /// Create an initial POST request.
    fn post(self, uri: &'a str) -> Result<surf::RequestBuilder>;
    /// Create an initial PUT request.
    fn put(self, uri: &'a str) -> Result<surf::RequestBuilder>;
}

/// Session representing the REST client.
pub struct Session {
    client:   Option<surf::Client>,
    auth:     BasicAuth,
    hostname: String,
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

impl<'a, 'b, 'c> NewSession<'a, 'b, 'c> for Session {
    fn new(hostname: &'a str, username: &'b str, password: &'c str) -> Self {
        let mut session = Self{
            client: None,
            auth: BasicAuth::new(username, password),
            hostname: hostname.to_owned(),
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
    fn delete(self, uri: &'a str) -> Result<surf::RequestBuilder> {
        match self.client() {
            Ok(client) => Ok(client.delete(uri)),
            Err(e) => Err(e)
        }
    }
    fn get(self, uri: &'a str) -> Result<surf::RequestBuilder> {
        match self.client() {
            Ok(client) => Ok(client.get(uri)),
            Err(e) => Err(e)
        }
    }
    fn patch(self, uri: &'a str) -> Result<surf::RequestBuilder> {
        match self.client() {
            Ok(client) => Ok(client.patch(uri)),
            Err(e) => Err(e)
        }
    }
    fn post(self, uri: &'a str) -> Result<surf::RequestBuilder> {
        match self.client() {
            Ok(client) => Ok(client.post(uri)),
            Err(e) => Err(e)
        }
    }
    fn put(self, uri: &'a str) -> Result<surf::RequestBuilder> {
        match self.client() {
            Ok(client) => Ok(client.put(uri)),
            Err(e) => Err(e)
        }
    }
}
