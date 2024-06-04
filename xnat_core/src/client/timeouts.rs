use std::time::Duration;

/// Periods of duration to configure different
/// timeouts when attempting to make request to
/// some REST host.
#[derive(Clone, Copy, Debug, Default)]
pub struct Timeouts {
    connect: Option<Duration>,
    read:    Option<Duration>,
}

impl Timeouts {
    /// Connection timeout limit.
    pub fn connect(&self) -> Duration {
        self
            .connect
            .unwrap_or(Duration::from_secs(super::APP_CONNECT_TIMEOUT))
    }

    /// Read timeout limit.
    pub fn read(&self) -> Duration {
        self.read.unwrap_or(self.connect())
    }

    /// Set the connection timeout limit in
    /// seconds.
    pub fn with_connect_secs(mut self, value: u64) -> Self {
        self.connect.clone_from(&Some(Duration::from_secs(value)));
        self
    }

    /// Set the read timeout limit in seconds.
    pub fn with_read_secs(mut self, value: u64) -> Self {
        self.read.clone_from(&Some(Duration::from_secs(value)));
        self
    }
}
