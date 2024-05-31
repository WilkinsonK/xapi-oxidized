use std::time::Duration;

use async_trait::async_trait;
use reqwest::{header::HeaderValue, redirect::Policy, Client, Method};

use oxinat_core::*;

#[cfg(feature = "core")]
pub extern crate oxinat_core;
#[cfg(feature = "derive")]
pub extern crate oxinat_derive;

static APP_CONNECT_TIMEOUT: u64 = 5;
static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION")
);

type RequestBuilderResult = anyhow::Result<reqwest::RequestBuilder>;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("`{0}` does not support method `{1}`")]
    UnsupportedMethod(Method, String)
}

#[derive(Clone, Version, AdminUri, AuthUri, ServicesUri, UsersUri)]
#[version(root_uri = "data", legacy = true)]
pub struct V1;
#[derive(Clone, Version, FullUri)]
#[version(root_uri = "xapi", data_uri = "data")]
pub struct V2;

#[derive(Clone, Copy, Debug, Default)]
pub struct Timeouts {
    pub connect: Option<Duration>,
    pub read:    Option<Duration>,
}

impl Timeouts {
    pub fn connect(&self) -> Duration {
        self
            .connect
            .unwrap_or(Duration::from_secs(APP_CONNECT_TIMEOUT))
    }

    pub fn read(&self) -> Duration {
        self.read.unwrap_or(self.connect())
    }
}

// TODO: impl std::mem::Drop for this struct.
// https://stackoverflow.com/questions/42910662/is-it-possible-in-rust-to-delete-an-object-before-the-end-of-scope
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Xnat<V: Version> {
    base_url:   reqwest::Url,
    session_id: Option<String>,
    timeouts:   Timeouts,
    use_secure: bool,
    version:    V,
}

impl<V: Version + Clone> Xnat<V> {
    pub fn configure(hostname: &str) -> XnatBuilder<V> {
        XnatBuilder::new(hostname)
    }

    pub fn new_client(&self) -> anyhow::Result<reqwest::Client> {
        let builder = reqwest::ClientBuilder::new()
            .connect_timeout(self.timeouts.connect())
            .read_timeout(self.timeouts.read())
            .redirect(Policy::default())
            .https_only(self.use_secure)
            .danger_accept_invalid_certs(!self.use_secure)
            .user_agent(APP_USER_AGENT);
        Ok(builder.build()?)
    }

    async fn request<UB: UriBuilder>(&self, method: Method, uri: &UB) -> RequestBuilderResult {
        let mut url = self.base_url.clone();
        url.set_path(&uri.build()?);

        let jar = reqwest::cookie::Jar::default();
        jar.add_cookie_str(
            &format!("JSESSIONID={}", self.session_id.as_ref().unwrap()),
            &url
        );

        let builder = self
            .client()?
            .request(method, url.to_owned());
        Ok(builder)
    }

    async fn method_is_supported<UB: UriBuilder>(&self, method: &Method, uri: &UB) -> anyhow::Result<bool> {
        let res = self
            .options(uri)
            .await?
            .send()
            .await?;

        let is_supported = |a: &HeaderValue| {
            !a.is_empty() && a.to_str().unwrap().contains(method.as_str())
        };
        Ok(res.headers().get("Allow").is_some_and(is_supported))
    }

    async fn request_if_supported<UB: UriBuilder>(&self, method: Method, uri: &UB) -> RequestBuilderResult {
        if self.method_is_supported(&method, uri).await? {
            self.request(method, uri).await
        } else {
            Err(ClientError::UnsupportedMethod(method, uri.to_string()).into())
        }
    }

    async fn head<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult {
        self.request_if_supported(Method::HEAD, uri).await
    }

    async fn delete<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult {
        self.request_if_supported(Method::DELETE, uri).await
    }

    async fn get<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult {
        self.request_if_supported(Method::GET, uri).await
    }

    async fn options<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult {
        self.request(Method::OPTIONS, uri).await
    }

    async fn post<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult {
        self.request_if_supported(Method::POST, uri).await
    }

    async fn put<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult {
        self.request_if_supported(Method::PUT, uri).await
    }
}

pub trait ClientCore {
    fn client(&self) -> anyhow::Result<reqwest::Client>;
}

impl<V: Version + Clone> ClientCore for Xnat<V> {
    fn client(&self) -> anyhow::Result<reqwest::Client> {
        self.new_client()
    }
}

pub trait ClientAuth: ClientCore {
    fn auth_uri(&self) -> BuildResult;
}

impl ClientAuth for Xnat<V1> {
    fn auth_uri(&self) -> BuildResult
    where
        V1: AuthUriLegacy,
    {
        self.version.auth_legacy().build_jsessionid()
    }
}

impl ClientAuth for Xnat<V2> {
    fn auth_uri(&self) -> BuildResult
    where
        V2: AuthUriLegacy,
    {
        self.version.auth_legacy().build_jsessionid()
    }
}

#[async_trait(?Send)]
pub trait ClientToken: ClientCore {
    async fn acquire(&mut self) -> anyhow::Result<()>;
}

#[async_trait(?Send)]
impl<V: Version + Clone> ClientToken for Xnat<V>
where
    Self: ClientAuth,
{
    async fn acquire(&mut self) -> anyhow::Result<()> {
        let res = self
            .post(&self.auth_uri()?)
            .await?
            .send()
            .await?;
        self.session_id.clone_from(&res.text().await.ok());
        todo!()
    }
}

#[allow(dead_code)]
pub struct XnatBuilder<V: Version> {
    hostname:   String,
    password:   Option<String>,
    timeouts:   Option<Timeouts>,
    username:   Option<String>,
    use_secure: bool,
    version:    Option<V>,
}

enum UrlKind<'a> {
    Base,
    Credentialed(&'a str, Option<&'a str>),
}

impl<V: Version + Clone> XnatBuilder<V> {
    pub fn new(hostname: &str) -> Self {
        XnatBuilder{
            hostname:   hostname.to_owned(),
            password:   None,
            timeouts:   None,
            username:   None,
            use_secure: false,
            version:    None
        }
    }

    pub fn use_secure(mut self, value: bool) -> Self {
        self.use_secure = value;
        self
    }

    pub fn with_hostname(mut self, hostname: &str) -> Self {
        hostname.clone_into(&mut self.hostname);
        self
    }

    pub fn with_password(mut self, password: &str) -> Self {
        self.password.clone_from(&Some(password.to_owned()));
        self
    }

    pub fn with_timeouts(mut self, timeouts: &Timeouts) -> Self {
        self.timeouts.clone_from(&Some(timeouts.to_owned()));
        self
    }

    pub fn with_username(mut self, username: &str) -> Self {
        self.username.clone_from(&Some(username.to_owned()));
        self
    }

    pub fn with_version(mut self, version: V) -> Self {
        self.version = Some(version);
        self
    }

    fn base_url(&self, kind: UrlKind) -> anyhow::Result<reqwest::Url> {
        let mut host = reqwest::Url::parse("http://stud")?;
        host.set_host(Some(&self.hostname.clone()))?;
        host.set_scheme(if self.use_secure {
            "https"
        } else {
            "http"
        }).unwrap();

        Ok(match kind {
            UrlKind::Base => host,
            UrlKind::Credentialed(u, p) => {
                host.set_password(p).unwrap();
                host.set_username(u).unwrap();
                host
            }
        })
    }

    fn version(&self) -> anyhow::Result<V> {
        Ok(self.version.as_ref().cloned().unwrap())
    }
}

pub trait ClientBuilder {
    type Client;

    fn build(&self) -> anyhow::Result<Self::Client>;
}

impl<V: Version + Clone> ClientBuilder for XnatBuilder<V> {
    type Client = Xnat<V>;

    fn build(&self) -> anyhow::Result<Self::Client> {
        Ok(Xnat {
            base_url:   self.base_url(UrlKind::Base)?,
            session_id: None,
            timeouts:   self.timeouts.unwrap_or_default(),
            use_secure: self.use_secure,
            version:    self.version()?,
        })
    }
}

#[async_trait(?Send)]
pub trait ClientBuilderToken: ClientBuilder
where
    Self::Client: ClientAuth,
{
    async fn acquire(&self) -> anyhow::Result<Self::Client>;
}

#[async_trait(?Send)]
impl<V: Version + Clone> ClientBuilderToken for XnatBuilder<V>
where
    Self::Client: ClientAuth,
{
    async fn acquire(&self) -> anyhow::Result<Self::Client> {
        let mut client = self.build()?;

        let creds  = UrlKind::Credentialed(
            &self.hostname,
            self.password.as_deref());
        let mut base_url = self.base_url(creds)?;
        base_url.set_path(&client.auth_uri()?);

        let res = client
            .client()?
            .post(base_url)
            .send()
            .await?;
        client.session_id.clone_from(&res.text().await.ok());

        Ok(client)
    }
}
