use async_trait::async_trait;
use reqwest::{header::HeaderValue, redirect::Policy, Method};

use oxinat_core::*;

use super::builder::{ClientBuilderCore, XnatBuilder};
use super::error::ClientError;
use super::timeouts::Timeouts;

type RequestBuilderResult = anyhow::Result<reqwest::RequestBuilder>;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Xnat<V: Version> {
    base_url:   reqwest::Url,
    session_id: Option<String>,
    timeouts:   Timeouts,
    use_secure: bool,
    version:    V,
}

impl<V: Version> Drop for Xnat<V> {
    fn drop(&mut self) {

        let mut url = self.base_url();
        // Setting the path manually since there's
        // no elegant way to implement ClientAuth
        // at this level that does not over
        // complicate this implementation.
        url.set_path("data/JSESSIONID");
        let client = self
            .new_client_builder()
            .build()
            .expect("must build client");

        // Need to either hijack the current
        // running scheduler or create a new one
        // to make the delete call.
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.spawn(async move {
            let res = client
                .delete(url)
                .send()
                .await
                .expect("must delete token");
            tokrel_validator(res)
                .await
                .expect("token must be invalidated");
        });
        rt.shutdown_background();
    }
}

impl<V: Version> Xnat<V> {
    /// Get the `JSESSIONID` cookie.
    pub fn get_session_id(&self) -> String {
        self.session_id.as_ref().unwrap().to_owned()
    }

    /// Sets the `JSESSIONID` cookie to this
    /// XNAT client.
    pub fn set_session_id(&mut self, value: &str) -> &Self {
        self.session_id.clone_from(&Some(value.to_string()));
        self
    }

    /// Pre-initialized base URL.
    fn base_url(&self) -> reqwest::Url {
        let url = self.base_url.clone();
        let jar = reqwest::cookie::Jar::default();

        if self.session_id.is_some() {
            jar.add_cookie_str(
                &format!("JSESSIONID={}", self.get_session_id()),
                &url
            );
        }
        url
    }

    /// Builds a blocking client needed for
    /// configuring a new client.
    fn new_client_builder(&self) -> reqwest::ClientBuilder {
        reqwest::ClientBuilder::new()
            .connect_timeout(self.timeouts.connect())
            .read_timeout(self.timeouts.read())
            .redirect(Policy::default())
            .https_only(self.use_secure)
            .danger_accept_invalid_certs(!self.use_secure)
            .user_agent(super::APP_USER_AGENT)
    }
}

/// Core behavior for a `Xnat` client. Defines
/// common use methods needed across all other
/// client traits.
pub trait ClientCore {
    type Version: Version + Clone;

    /// Constructs a REST client.
    fn client(&self) -> anyhow::Result<reqwest::Client>;
    /// Initialize an `XnatBuilder` allowing
    /// configuration of an XNAT client.
    fn configure(hostname: &str) -> XnatBuilder<Self::Version>;
    /// Create a new instance of an XNAT client.
    fn new(base_url: &reqwest::Url, timeouts: &Option<Timeouts>, use_secure: bool, version: &Self::Version) -> Self;
}

impl<V: Version + Clone> ClientCore for Xnat<V> {
    type Version = V;

    fn client(&self) -> anyhow::Result<reqwest::Client> {
        Ok(self.new_client_builder().build()?)
    }

    fn configure(hostname: &str) -> XnatBuilder<Self::Version> {
        XnatBuilder::new(hostname)
    }

    fn new(base_url: &reqwest::Url, timeouts: &Option<Timeouts>, use_secure: bool, version: &Self::Version) -> Self {
        Self {
            base_url:   base_url.to_owned(),
            session_id: None,
            timeouts:   timeouts.unwrap_or_default(),
            use_secure,
            version: version.to_owned(),
        }
    }
}

/// Methods necessary for performing authorization
/// tasks.
pub trait ClientAuth: ClientCore {
    /// Construct the URI endpoint to make
    /// requests of token/session_id acquisition.
    fn auth_uri(&self) -> BuildResult;
}

impl<V: Version + Clone> ClientAuth for Xnat<V>
where
    Self::Version: AuthUriLegacy,
{
    fn auth_uri(&self) -> BuildResult {
        self.version.auth_legacy().build_jsessionid()
    }
}

#[async_trait(?Send)]
pub trait ClientREST: ClientCore {
    /// Initialize a `DELETE` request. Is
    /// successful if the given URI endpoint
    /// supports `DELETE`.
    async fn delete<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult;
    /// Initialize a `GET` request. Is successful
    /// if the given URI endpoint supports `GET`.
    async fn get<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult;
    /// Initialize a `HEAD` request. Is successful
    /// if the given URI endpoint supports `HEAD`.
    async fn head<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult;
    /// URI endpoint supports the REST method.
    async fn method_is_supported<UB: UriBuilder>(&self, method: &Method, uri: &UB) -> anyhow::Result<bool>;
    /// Initialize a `OPTION` request.
    async fn options<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult;
    /// Initialize a `POST` request. Is successful
    /// if the given URI endpoint supports `POST`.
    async fn post<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult;
    /// Initialize a `PUT` request. Is successful
    /// if the given URI endpoint supports `PUT`.
    async fn put<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult;
    /// Pre-initializes a `RequestBuilder` for
    /// further configuration and sends to the
    /// host.
    async fn request<UB: UriBuilder>(&self, method: Method, uri: &UB) -> RequestBuilderResult;
    /// Makes a request for some URI endpoint if
    /// the method is supported.
    async fn request_if_supported<UB: UriBuilder>(&self, method: Method, uri: &UB) -> RequestBuilderResult;
}

#[async_trait(?Send)]
impl<V: Version + Clone> ClientREST for Xnat<V> {
    async fn delete<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult {
        self.request_if_supported(Method::DELETE, uri).await
    }

    async fn get<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult {
        self.request_if_supported(Method::GET, uri).await
    }

    async fn head<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult {
        self.request_if_supported(Method::HEAD, uri).await
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

    async fn options<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult {
        self.request(Method::OPTIONS, uri).await
    }

    async fn post<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult {
        self.request_if_supported(Method::POST, uri).await
    }

    async fn put<UB: UriBuilder>(&self, uri: &UB) -> RequestBuilderResult {
        self.request_if_supported(Method::PUT, uri).await
    }

    async fn request<UB: UriBuilder>(&self, method: Method, uri: &UB) -> RequestBuilderResult {
        let mut url = self.base_url();
        url.set_path(&uri.build()?);

        let builder = self
            .client()?
            .request(method, url.to_owned());
        Ok(builder)
    }

    async fn request_if_supported<UB: UriBuilder>(&self, method: Method, uri: &UB) -> RequestBuilderResult {
        if self.method_is_supported(&method, uri).await? {
            self.request(method, uri).await
        } else {
            Err(ClientError::UnsupportedMethod(method, uri.to_string()).into())
        }
    }
}

/// Methods necessary for managing auth tokens
/// from an XNAT host.
#[async_trait(?Send)]
pub trait ClientToken: ClientCore {
    /// Acquire an auth token from the XNAT host.
    async fn acquire(&mut self) -> anyhow::Result<()>;
    /// Invalidates the auto token.
    async fn release(&mut self) -> anyhow::Result<()>;
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
        tokacq_validator(res).await.map(|token| {
            self.set_session_id(&token);
        })
    }

    async fn release(&mut self) -> anyhow::Result<()> {
        let res = self
            .delete(&self.auth_uri()?)
            .await?
            .send()
            .await?;
        tokrel_validator(res).await.map(|r| {
            self.session_id.take();
            r
        })
    }
}

/// Helper function for token acquisition to
/// validate that the transaction was successful.
pub async fn tokacq_validator(res: reqwest::Response) -> anyhow::Result<String> {
    let status = res.status();
    if status.is_success() {
        Ok(res.text().await?)
    } else if status.is_client_error() {
        Err(ClientError::AuthFailure(status.as_u16()).into())
    } else {
        Err(ClientError::ServerFailure(status.as_u16()).into())
    }
}

/// Helper function for token relinquishment to
/// validate that the transaction was successful.
pub async fn tokrel_validator(res: reqwest::Response) -> anyhow::Result<()> {
    let status = res.status();
    if status.is_success() {
        Ok(())
    } else if status.is_client_error() {
        Err(ClientError::DeauthFailure(status.as_u16()).into())
    } else {
        Err(ClientError::ServerFailure(status.as_u16()).into())
    }
}
