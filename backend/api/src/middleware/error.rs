use castle_api::async_trait;

use envoy_http::{Middleware, Response, StatusCode};
use serde_json::json;
pub struct ErrorMiddleware;

#[async_trait]
impl Middleware for ErrorMiddleware {
    async fn handle(&self, ctx: &mut envoy_http::Context, next: envoy_http::Next) -> envoy_http::Result {
        match next.run(ctx).await {
            Ok(response) => Ok(response),
            Err(error) => {
                let res = serde_json::to_string_pretty(&json!({
                    "data": {},
                    "errors": vec![error.to_string()]
                }))?; // this ? could fail but there's no real way to handle it

                let mut response = Response::new(res.into());
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

                Ok(response)
            }
        }
    }
}