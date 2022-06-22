use std::net::SocketAddr;

use api::create_app;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = create_app().await;

    app.listen(SocketAddr::from(([0, 0, 0, 0], 80))).await.unwrap();
}
