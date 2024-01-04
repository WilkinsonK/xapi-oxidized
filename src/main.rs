use oxidized_xnat_rest::{self as oxr, NewSession};
use tokio;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let _ = oxr::Session::from_host("");
    Ok(())
}
