use oxidized_xnat_rest::{self as oxr, NewSession, SessionREST};
use oxidized_xnat_rest::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Pass a base URL as the host name in this
    // method call.
    let session = oxr::Session::from_host("");
    // Build and send a new request using `surf`
    // API.
    let mut req = session.get("xapi/users/username")?.await?;
    // Attempt to parse the response body.
    println!("from body: [{}]({})", req.body_string().await?, req.status());
    Ok(())
}
