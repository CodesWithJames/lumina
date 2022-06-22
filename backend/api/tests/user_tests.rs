use serde_json::json;

use crate::shared::query;

mod shared;

#[tokio::test]
async fn can_create_user() {
    let res = query("message {
            create_user(
                first_name: \"John\",
                last_name: \"Doe\",
                email: \"john@example.com\",
                password: \"password\"
            )
        }").await;
    assert_eq!(res["errors"], json!([]));
}

#[tokio::test]
async fn user_can_log_in() {
    let res = query("message {
            login(
                email: \"john@example.com\",
                password: \"password\"
            )
        }").await;

    println!("{:?}", res);
    assert!(res["data"]["login"].is_string())
}
