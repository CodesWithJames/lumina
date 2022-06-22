use serde_json::json;

use crate::shared::query;

mod shared;

#[tokio::test]
async fn ping() {
    assert_eq!(
        query("message {
            ping
        }").await,
        json!({
            "data": {
                "ping": "pong"
            },
            "errors": []
        })
    );
}