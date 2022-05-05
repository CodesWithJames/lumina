use tide::Request;
use tracing::info;



pub async fn graph_handler(request: Request<()>) -> tide::Result {
    Ok("hello".into())
}

struct Query {
    query: String,
}