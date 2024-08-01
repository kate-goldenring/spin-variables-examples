use spin_sdk::{
    http::{IntoResponse, Method, Request, Response},
    http_component, variables,
};

#[http_component]
async fn handle_api_call_with_token(_req: Request) -> anyhow::Result<impl IntoResponse> {
    let token = variables::get("token")?;
    let api_uri = variables::get("api_uri")?;
    let version = variables::get("version")?;
    let versioned_api_uri = format!("{}/{}", api_uri, version);
    let request = Request::builder()
        .method(Method::Get)
        .uri(versioned_api_uri)
        .header("Authorization", format!("Bearer {}", token))
        .build();
    let response: Response = spin_sdk::http::send(request).await?;
    // Do something with the response ...
    Ok(Response::builder()
        .status(200)
        .build())
}
