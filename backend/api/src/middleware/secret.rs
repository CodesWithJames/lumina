

use std::sync::Arc;

use castle_api::async_trait;
use envoy_http::Middleware;
use secrets::{BackendSecrets, get_secret, SECRET_NAME};

pub struct SecretMiddleware {
    secret: Arc<BackendSecrets>
}

impl SecretMiddleware {
    pub async fn create() -> Self {
        Self {
            secret: Arc::new(get_secret::<BackendSecrets>(SECRET_NAME).await)
        }
    }
}

#[async_trait]
impl Middleware for SecretMiddleware {
    async fn handle(&self, ctx: &mut envoy_http::Context, next: envoy_http::Next) -> envoy_http::Result {
        ctx.insert(self.secret.clone());

        next.run(ctx).await
    }
}