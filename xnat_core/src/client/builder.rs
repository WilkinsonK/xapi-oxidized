use async_trait::async_trait;

use crate::Version;
use super::timeouts::Timeouts;
use super::clients::{ClientAuth, ClientCore, Xnat};

/// A building pattern type meant for constructing
/// an XNAT client.
#[allow(dead_code)]
pub struct XnatBuilder<V: Version> {
    hostname:   String,
    password:   Option<String>,
    timeouts:   Option<Timeouts>,
    username:   Option<String>,
    use_secure: bool,
    version:    Option<V>,
}

/// Internal usage only. Dictates how a URL should
/// be constructed.
enum UrlKind<'a> {
    Base,
    Credentialed(&'a str, Option<&'a str>),
}

impl<V: Version + Clone> XnatBuilder<V> {
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

/// Core methods required by all subsequent
/// client building traits.
pub trait ClientBuilderCore {
    type Client;

    /// Attempt to build a client from this
    /// builder.
    /// 
    /// ```no_compile
    /// use oxinat_core::*;
    /// 
    /// #[derive(Clone, Version, FullUri)]
    /// #[version(root_uri = "xapi", data_uri = "data")]
    /// struct MyVersion;
    /// 
    /// let builder = XnatBuilder::new("xnat.host.org")
    ///     .with_version(MyVersion)
    ///     .with_password("my-password")
    ///     .with_username("my-username");
    /// ```
    fn build(&self) -> anyhow::Result<Self::Client>;
    /// Initialize a new builder instance.
    fn new(hostname: &str) -> Self;
}

impl<V: Version + Clone> ClientBuilderCore for XnatBuilder<V> {
    type Client = Xnat<V>;

    fn build(&self) -> anyhow::Result<Self::Client> {
        Ok(Xnat::new(
            &self.base_url(UrlKind::Base)?,
            &self.timeouts,
            self.use_secure,
            &self.version()?,
        ))
    }

    fn new(hostname: &str) -> Self {
        XnatBuilder{
            hostname:   hostname.to_owned(),
            password:   None,
            timeouts:   None,
            username:   None,
            use_secure: false,
            version:    None
        }
    }
}

/// Trait dictates the methods used to customize
/// how the resulting clients are constructed.
pub trait ClientBuilderAttrs: ClientBuilderCore {
    type Version: Version + Clone;

    /// Set whether constructed clients should
    /// use secure protocols and verify SSL certs.
    fn use_secure(self, value: bool) -> Self;
    /// Set the host name that will be assigned
    /// to constructed clients.
    fn with_hostname(self, hostname: &str) -> Self;
    /// Set the auth password to be used for
    /// token acquisition for constructed clients.
    fn with_password(self, password: &str) -> Self;
    /// Set the timeout values (connect & read) to
    /// be assigned to constructed clients.
    fn with_timeouts(self, timeouts: &Timeouts) -> Self;
    /// Set the auth username to be used for
    /// token acquisition for constructed clients.
    fn with_username(self, username: &str) -> Self;
    /// Set the API version representation to be
    /// assigned to constructed clients.
    fn with_version(self, version: Self::Version) -> Self;
}

impl<V: Version + Clone> ClientBuilderAttrs for XnatBuilder<V> {
    type Version = V;

    fn use_secure(mut self, value: bool) -> Self {
        self.use_secure = value;
        self
    }

    fn with_hostname(mut self, hostname: &str) -> Self {
        hostname.clone_into(&mut self.hostname);
        self
    }

    fn with_password(mut self, password: &str) -> Self {
        self.password.clone_from(&Some(password.to_owned()));
        self
    }

    fn with_timeouts(mut self, timeouts: &Timeouts) -> Self {
        self.timeouts.clone_from(&Some(timeouts.to_owned()));
        self
    }

    fn with_username(mut self, username: &str) -> Self {
        self.username.clone_from(&Some(username.to_owned()));
        self
    }

    fn with_version(mut self, version: Self::Version) -> Self {
        self.version = Some(version);
        self
    }
}

#[async_trait(?Send)]
pub trait ClientBuilderToken: ClientBuilderCore
where
    Self::Client: ClientAuth,
{
    /// Brokers the acquisition of a `token` via
    /// user authentication when constructing a
    /// new client.
    /// 
    /// ```no_compile
    /// use oxinat_core::*;
    /// 
    /// #[derive(Clone, Version, FullUri)]
    /// #[version(root_uri = "xapi", data_uri = "data")]
    /// struct MyVersion;
    /// 
    /// let builder = XnatBuilder::new("xnat.host.org")
    ///     .with_version(MyVersion)
    ///     .with_password("my-password")
    ///     .with_username("my-username");
    /// 
    /// let client = builder.acquire().await?;
    /// ```
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

        super::clients::tokacq_validator(res)
            .await
            .map(|token| {
                client.set_session_id(&token);
                client
            })
    }
}
