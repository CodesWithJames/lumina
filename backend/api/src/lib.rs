
mod middleware;
mod graph;
mod models;
mod errors;

pub async fn create_app() -> envoy_http::Server {
    let mut app = envoy_http::new();

    app.with(middleware::ErrorMiddleware);
    app.with(middleware::LogMiddleware);
    app.with(middleware::CORSMiddleware);
    app.with(middleware::NamespaceMiddleware::create().await);
    app.with(middleware::SecretMiddleware::create().await);
    app.with(middleware::MongoDBMiddleware::new());
    app.with(middleware::AuthenticationMiddleware);

    app.at("/graph")
        .post(graph::CastleEndpoint::create().await);
    // 404
    app.at("*")
        .with(middleware::NotFoundMiddleware);

    app
}
