use xapi_oxidized::{self as xapi, NewSession, SessionREST};
use xapi_oxidized::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Pass a base URL as the host name in this
    // method call.
    let mut session = xapi::Session::from_host("");
    // Build and send a new request using `surf`
    // API.
    let mut req = session.get()?.await?;
    // Attempt to parse the response body.
    println!("from body: [{}]({})", req.body_string().await?, req.status());
    Ok(())
}
