mod middleware;
mod graph;
mod models;

use models::User;

struct AppState {
    user: Option<User>,
    // db: Option<FractalClient>
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut app = tide::new();

    app.with(middleware::ErrorMiddleware);
    app.with(middleware::LogMiddleware);
    // app.with(middleware::AuthenticationMiddleware);

    app.at("/graph")
        .post(graph::graph_handler);

    app.listen("0.0.0.0:80").await.unwrap();
}
