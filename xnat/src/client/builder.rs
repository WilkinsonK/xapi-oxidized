use async_trait::async_trait;

use oxinat_core::*;

use super::timeouts::Timeouts;
use super::clients::{ClientAuth, ClientCore, Xnat};

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
        Ok(Xnat::new(
            &self.base_url(UrlKind::Base)?,
            &self.timeouts,
            self.use_secure,
            &self.version()?,
        ))
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
        client.set_session_id(&res.text().await.unwrap());
        Ok(client)
    }
}
