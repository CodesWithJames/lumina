use std::sync::Arc;

use castle_api::async_trait;
use envoy_http::Middleware;
use secrets::BackendSecrets;

use crate::models::User;

pub struct AuthenticationMiddleware;

#[async_trait]
impl Middleware for AuthenticationMiddleware {
    /// AuthenticationMiddleware adds a user state into the request context.
    ///
    /// ## Algorithm
    /// let user be match `auth_token` header
    ///     Some(auth_token) => authenticate_from_token(auth_token, secret)
    ///     None => {}
    async fn handle(
        &self,
        ctx: &mut envoy_http::Context,
        next: envoy_http::Next,
    ) -> envoy_http::Result {
        let headers = ctx.borrow::<envoy_http::HeaderMap>();

        match headers.get("auth_token") {
            Some(auth_token) => {
                let secret = ctx.borrow::<Arc<BackendSecrets>>();
                let user = User::authenticate_from_token(auth_token.to_str()?, &secret.jwt_secret).await?;
                ctx.insert(user);
            }
            None => {}
        }

        next.run(ctx).await
    }
}
