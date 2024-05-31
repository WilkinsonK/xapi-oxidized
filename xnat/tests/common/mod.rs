use std::sync::Once;

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
pub fn init() {
    INIT.call_once(|| {
        dotenv::dotenv().ok();
    })
}
