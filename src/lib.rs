use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Params, Request, Response, Router},
    http_component,
};

/// A Spin HTTP component that internally routes requests.
#[http_component]
fn handle_route(req: Request) -> Response {
    let mut router = Router::new();
    router.get("/hello/:name", api::hello_name);
    router.any_async("/*", api::hello);
    router.handle(req)
}

mod api {
    use super::*;

    pub fn hello_name(_req: Request, params: Params) -> Result<impl IntoResponse> {
        let name = params.get("name").expect("missing name");
        let hello = String::from("Hello ");
        Ok(Response::new(200, hello + name))
    }

    pub async fn hello(_req: Request, _params: Params) -> Result<impl IntoResponse> {
        Ok(Response::new(200, "Hello from WASM!"))
    }
}
