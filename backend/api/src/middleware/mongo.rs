use std::sync::Arc;

use castle_api::async_trait;
use mongodb::Client;
use secrets::BackendSecrets;
use envoy_http::Middleware;
use tokio::sync::RwLock;

use crate::errors::{mongo_error::MongoError};


pub struct MongoDBMiddleware {
    client: RwLock<Option<Client>>,
}

#[async_trait]
impl Middleware for MongoDBMiddleware {
    async fn handle(&self, ctx: &mut envoy_http::Context, next: envoy_http::Next) -> envoy_http::Result {
        let client = self.client.read().await.clone();
        let client = match client {
            Some(client) => client,
            None => {
                let secret = &ctx.borrow::<Arc<BackendSecrets>>()
                    .mongo;

                let uri = format!(
                    "mongodb+srv://{}:{}@{}/test?retryWrites=true&w=majority",
                    secret.username,
                    secret.password,
                    secret.host,
                );

                let client = Client::with_uri_str(&uri).await
                        .map_err(|err| MongoError(err.to_string()))?;

                *self.client.write().await = Some(client.clone());

                client
            }
        };

        ctx.insert(client);

        next.run(ctx).await
    }
}

impl MongoDBMiddleware {
    pub fn new() -> Self {
        Self {
            client: RwLock::new(None),
        }
    }
}