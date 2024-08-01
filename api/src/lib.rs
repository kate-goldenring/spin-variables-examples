use anyhow::Context;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::{
    http_component, variables,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let token = variables::get("token").context("could not get 'token' variable")?;
    let auth = req.header("Authorization").context("expected auth header")?.as_str().context("expected auth header to be a string")?;
    if auth != format!("Bearer {}", token) {
        return Ok(Response::builder()
            .status(401)
            .header("content-type", "text/plain")
            .body("Unauthorized")
            .build());
    }
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Authorized")
        .build())
}
