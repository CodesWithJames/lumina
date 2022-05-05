use k8s_openapi::api::core::v1::Secret;
use rand::Rng;
use kube::{Api, Client, api::PostParams};
use serde_json::json;

/// Get a kubernetes client
async fn get_client() -> Client {
    let client = Client::try_default().await.unwrap();
    client
}

/// Get our pods current namespace
async fn get_namespace() -> String {
    let config = kube::config::Config::infer().await.unwrap();
    config.default_namespace
}

/// This will check if a secret already exists in the kubernetes cluster, if it does it will return the secret, if not it will create a new one
/// and atomically update the secret in the cluster (using ResourceVersion)
pub async fn get_or_generate_secret() -> String {
    let client = get_client().await;
    let namespace = get_namespace().await;

    let secret_name = "jwt-secret";

    let secret_api: Api<Secret> = Api::namespaced(client, &namespace);

    loop {
        match secret_api.get(secret_name).await {
            Ok(Secret { data: Some(data), .. }) => return String::from_utf8_lossy(&data.get("secret").unwrap().0).to_string(),
            _ => {
                let secret = generate_random_secret();
                let secret_body = create_secret_data(&secret).await;
                match secret_api.create(&PostParams::default(),&secret_body).await {
                    Ok(_) => return secret,
                    Err(kube::Error::Api(kube::core::ErrorResponse { reason, .. })) if reason == "AlreadyExists" => continue,
                    Err(e) => panic!("Error getting secret: {:?}", e),
                }
            }
        }
    }
}

/// Create the secret data
async fn create_secret_data(secret: &str) -> Secret {
    let namespace = get_namespace().await;

    let secret_body: Secret = serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Secret",
        "metadata": {
            "name": "jwt-secret",
            "namespace": &namespace
        },
        "data": {
            "secret": base64::encode(&secret),
        }
    })).unwrap();

    secret_body
}

fn generate_random_secret() -> String {
    let mut rand = rand::thread_rng();

    // generate 80 random bytes
    let mut secret = [0u8; 80];
    rand.fill(&mut secret as &mut [u8]);


    // print the secret
    secret.map(|x| format!("{:02X?}", x)).join("")
}

#[tokio::test]
async fn can_get_namespace() {
    let namespace = get_namespace().await;

    println!("{}", namespace);
}


#[tokio::test]
async fn can_get_or_generate_secret() {
    let secret = get_or_generate_secret().await;

    println!("{}", secret);
}