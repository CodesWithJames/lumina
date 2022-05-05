use tide::{Request, Next, Middleware, utils::async_trait};
use tracing::{span, Level, Instrument};

pub struct LogMiddleware;

#[async_trait]
impl Middleware<()> for LogMiddleware {
    async fn handle(&self, req: Request<()>, next: Next<'_, ()>) -> tide::Result {
        let span = span!(Level::INFO, "req", uuid = %uuid::Uuid::new_v4().to_string());
        {
            let _enter = span.enter();
            tracing::event!(Level::INFO, "{} {}", req.method(), req.url());
        }
        Ok(next.run(req).instrument(span).await)
    }
}