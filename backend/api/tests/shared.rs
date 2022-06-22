use envoy_http::{Request, Method, Body, Response, body::to_bytes};
use serde_json::{Value};

pub async fn query(query: &str) -> Value {
    let app = api::create_app().await;

    let req = Request::builder()
        .method(Method::POST)
        .header("Origin", "https://lumina.earth")
        .uri("/graph")
        .body(Body::from(query.to_string()))
        .unwrap();

    let res: Response<Body> = app.respond(req).await.unwrap();

    let body = to_bytes(res.into_body()).await.unwrap();

    serde_json::from_slice::<Value>(&body[..]).unwrap()
}