use crate::Session;

/// Legacy XAPI component. Used for accessing data
/// from the XNAT.
const DATA_URI_PREFIX: &str = "data";
/// Used for making calls to operations provided
/// by the XAPI.
const XAPI_URI_PREFIX: &str = "xapi";

/// Implements the core methods used to read
/// a `Broker`'s attributes.
pub trait BrokerAttributes {
    /// Get the API data prefix.
    fn data_prefix<'a>(self) -> &'a str;
    /// Create a new instance of a broker.
    fn new(sxn: Session) -> Self;
    /// Get the API root prefix.
    fn root_prefix<'a>(self) -> &'a str;
    /// Get the API client session.
    fn session<'a>(self) -> Session;
    /// Get the API broker version.
    fn version<'a>(self) -> &'a str;
}

/// Implements the core methods needed to manage
/// some API version.
pub trait BrokerVersion
where
    Self: Sized,
{
    /// Data prefix used to making calls to the
    /// host that require access to XNAT data or
    /// use of the legacy API.
    #[inline]
    fn data_prefix<'a>(self) -> &'a str {
        DATA_URI_PREFIX
    }
    /// Creates a new instance of this struct.
    fn new() -> Self;
    /// URI prefix used for making non-legacy
    /// calls to the target XNAT.
    #[inline]
    fn root_prefix<'a>(self) -> &'a str {
        XAPI_URI_PREFIX
    }
    fn version<'a>(self) -> &'a str;
}

pub struct Latest;
pub struct Legacy;

/// The controlling object which builds and
/// executes REST calls. Wraps around API version
/// objects to use their interface to inform how
/// calls should be made.
pub struct Broker<T: BrokerVersion> {
    session: Session,
    version: T,
}

impl<T: BrokerVersion> BrokerAttributes for Broker<T> {
    /// Get the API data prefix.
    fn data_prefix<'a>(self) -> &'a str {
        self.version.data_prefix()
    }
    /// Create a new instance of a broker.
    fn new(sxn: Session) -> Self {
        Self{session: sxn, version: T::new() }
    }
    /// Get the API root prefix.
    fn root_prefix<'a>(self) -> &'a str {
        self.version.root_prefix()
    }
    /// Get the API client session.
    fn session<'a>(self) -> Session {
        self.session
    }
    /// Get the API broker version.
    fn version<'a>(self) -> &'a str {
        self.version.version()
    }
}

impl BrokerVersion for Latest {
    fn new() -> Self {
        Self{}
    }
    fn version<'a>(self) -> &'a str {
        "Latest"
    }
}

impl BrokerVersion for Legacy {
    fn new() -> Self {
        Self{}
    }
    fn root_prefix<'a>(self) -> &'a str {
        self.data_prefix()
    }
    fn version<'a>(self) -> &'a str {
        "Legacy"
    }
}
