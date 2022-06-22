use std::str::FromStr;

use castle_api::async_trait;

use envoy_http::{Middleware, Uri, http::{HeaderValue, header}, Response, StatusCode, Body, HeaderMap};
pub struct CORSMiddleware;

#[async_trait]
impl Middleware for CORSMiddleware {
    async fn handle(&self, ctx: &mut envoy_http::Context, next: envoy_http::Next) -> envoy_http::Result {
        // we need to check the origin header to make sure it's valid
        let headers = ctx.borrow::<envoy_http::HeaderMap>();

        match headers.get("origin").cloned() {
            Some(origin) => {
                let origin_uri = Uri::from_str(origin.to_str()?)?;

                // we want to strip any subdomains from the host and get the last 2 parts (domain and TLD)
                // there is probably a better way to do this
                let host_parts = origin_uri.host()
                    .map(|host| host
                        .split('.')
                        .rev()
                        .take(2)
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev()
                        .collect::<Vec<_>>()
                        .join("."));

                // if this is an options request, we want to handle it
                let mut res = if ctx.borrow::<envoy_http::Method>() == &envoy_http::Method::OPTIONS {
                    match host_parts {
                        Some(host) if host == "lumina.earth" => Response::builder()
                            .status(StatusCode::OK)
                            .header(header::ACCESS_CONTROL_ALLOW_METHODS, "*")
                            .header(header::ACCESS_CONTROL_ALLOW_HEADERS, "*")
                            .header(header::ACCESS_CONTROL_MAX_AGE, "600")
                            .body(Body::empty())?,
                        _ => Err(anyhow::anyhow!("Invalid origin"))?,
                    }
                } else {
                    next.run(ctx).await?
                };

                res.headers_mut().append(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);

                Ok(res)
            },
            None => next.run(ctx).await
        }
    }
}