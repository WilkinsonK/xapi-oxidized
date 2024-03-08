use tokio;

use xapi_oxidized::{self as xapi, CoreResource, ApiResource, ArgRequiredBy::Child, WithResource};

#[tokio::main]
async fn main() -> xapi::Result<()> {
    let mut sessions: ApiResource<'_, String> = xapi::ApiResource::new("experiments");
    let mut subjects = xapi::ApiResource::new("subjects")
        .with_arg_required(Child)
        .with_child(&mut sessions)?;
    let projects = xapi::ApiResource::new("projects")
        .with_arg_required(Child)
        .with_child(&mut subjects);

    println!("route: {projects:#?}");
    Ok(())
}
