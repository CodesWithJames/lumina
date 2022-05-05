use tide::{Request, Next, Middleware, utils::async_trait};
use tracing::{Level};
pub struct AuthenticationMiddleware;

#[async_trait]
impl Middleware<()> for AuthenticationMiddleware {

    /// AuthenticationMiddleware adds a user state into the request context.
    ///
    /// ## Algorithm
    /// let state be match `auth_token` header
    ///     Some(auth_token) => load_user_from_token(auth_token)
    ///     None => None
    async fn handle(&self, request: Request<()>, next: Next<'_, ()>) -> tide::Result {
        unimplemented!()
    }
}
