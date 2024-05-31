use std::time::Duration;

#[derive(Clone, Copy, Debug, Default)]
pub struct Timeouts {
    pub connect: Option<Duration>,
    pub read:    Option<Duration>,
}

impl Timeouts {
    pub fn connect(&self) -> Duration {
        self
            .connect
            .unwrap_or(Duration::from_secs(super::APP_CONNECT_TIMEOUT))
    }

    pub fn read(&self) -> Duration {
        self.read.unwrap_or(self.connect())
    }
}
