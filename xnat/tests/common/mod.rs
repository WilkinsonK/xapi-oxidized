use std::sync::Once;

use oxinat::{
    ClientBuilderAttrs,
    ClientBuilderToken,
    ClientCore,
    Xnat,
    V2
};
use oxinat_core::client::timeouts::Timeouts;

#[allow(dead_code)]
static INIT: Once = Once::new();

#[allow(dead_code)]
pub const ENV_OXINAT_TEST_HOSTNAME: &str = "OXINAT_TEST_HOSTNAME";
#[allow(dead_code)]
pub const ENV_OXINAT_TEST_USERNAME: &str = "OXINAT_TEST_USERNAME";
#[allow(dead_code)]
pub const ENV_OXINAT_TEST_PASSWORD: &str = "OXINAT_TEST_PASSWORD";

#[allow(dead_code)]
pub fn env_hostname() -> String {
    std::env::var(ENV_OXINAT_TEST_HOSTNAME).unwrap()
}

#[allow(dead_code)]
pub fn env_username() -> String {
    std::env::var(ENV_OXINAT_TEST_USERNAME).unwrap()
}

#[allow(dead_code)]
pub fn env_password() -> String {
    std::env::var(ENV_OXINAT_TEST_PASSWORD).unwrap()
}

#[allow(dead_code)]
pub async fn request_client() -> Xnat<V2> {
    oxinat::Xnat::configure(&env_hostname())
        .use_secure(true)
        .with_password(&env_password())
        .with_username(&env_username())
        .with_timeouts(&Timeouts::default().with_connect_secs(300))
        .with_version(oxinat::V2)
        .acquire()
        .await
        .unwrap()
}

#[allow(dead_code)]
pub fn init() {
    INIT.call_once(|| {
        dotenv::dotenv().ok();
        flexi_logger::Logger::with(flexi_logger::LogSpecification::debug())
            .start()
            .unwrap();
    })
}
