

use std::sync::Arc;

use castle_api::{async_trait, Castle, types::State};
use envoy_http::{Endpoint, Response, Body};
use mongodb::Client as MongoClient;
use secrets::BackendSecrets;
use serde_json::json;

use crate::{models::{Root, User}, middleware::KubernetesNamespace};

pub struct CastleEndpoint {
    pub castle: Castle,
}

#[async_trait]
impl Endpoint for CastleEndpoint {
    async fn call(&self, ctx: &mut envoy_http::Context) -> envoy_http::Result {
        Ok(Response::new(Body::from(graph_handler(ctx, &self.castle).await?)))
    }
}

impl CastleEndpoint {
    pub async fn create() -> Self {
        Self {
            castle: castle_api::castle::CastleBuilder::new(Root)
                .build()
                .unwrap(),
        }
    }
}

pub async fn graph_handler(
    ctx: &mut envoy_http::Context,
    castle: &Castle,
) -> Result<String, anyhow::Error> {
    let body = envoy_http::body::to_bytes(ctx.take::<Body>()).await?;
    let query = String::from_utf8_lossy(&body);

    // Set up the castle context
    let mut state = State::new();
    ctx.try_take::<User>().map(|user| state.insert(user));
    ctx.try_take::<Arc<BackendSecrets>>().map(|secret| state.insert(secret));
    ctx.try_take::<MongoClient>().map(|client| state.insert(client));
    ctx.try_take::<KubernetesNamespace>().map(|secret| state.insert(secret));

    let json = match castle.run_message(&query, &state).await {
        Ok((data, errors)) => json!({
            "data": data,
            "errors": errors
                .into_iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<_>>()
        }),
        Err(e) => json!({
            "data": {},
            "errors": vec![e.to_string()]
        }),
    };

    match serde_json::to_string_pretty(&json) {
        Ok(json) => Ok(json.into()),
        Err(e) => Err(e.into()),
    }
}
