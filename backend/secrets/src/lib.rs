use k8s_openapi::{api::core::v1::Secret, serde::{Serialize, Deserialize, de::DeserializeOwned}};
use rand::Rng;
use kube::{Api, Client};
use serde_json::json;

const DEV_NAMESPACE: &str = "dev";
pub const SECRET_NAME: &str = "backend-secrets";

#[derive(Debug, Serialize, Deserialize)]
pub struct BackendSecrets {
    pub jwt_secret: String,
    pub mongo: MongoSecrets,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MongoSecrets {
    pub username: String,
    pub password: String,
    pub host: String,
}

/// Get a kubernetes client
async fn get_client() -> Client {
    let client = Client::try_default().await.unwrap();
    client
}

/// Get our pods current namespace
pub async fn get_namespace() -> String {
    let config = kube::config::Config::infer().await.unwrap();
    config.default_namespace
}

/// Panics if the secret does not exist
pub async fn get_secret<D: Serialize + DeserializeOwned>(secret_name: &str) -> D {
    let client = get_client().await;
    let secret_api: Api<Secret> = Api::namespaced(client, DEV_NAMESPACE);

    match secret_api.get(secret_name).await {
        Ok(Secret { data: Some(data), .. }) => serde_json::from_slice(&data.get("secret").unwrap().0).unwrap(),
        Err(e) => panic!("Error getting secret: {:?}", e),
        _ => panic!("Secret had missing data"),
    }
}

async fn create_kube_secret_object<S: Serialize>(secret: S, name: &str, namespace: &str) -> Secret {
    serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Secret",
        "metadata": {
            "name": &name,
            "namespace": &namespace
        },
        "data": {
            "secret": base64::encode(&serde_json::to_string(&secret).unwrap()),
        }
    })).unwrap()
}

fn generate_random_hex<const LEN: usize>() -> String {
    let mut rand = rand::thread_rng();

    let mut secret = [0u8; LEN];
    rand.fill(&mut secret as &mut [u8]);

    // print the secret
    secret.map(|x| format!("{:02X?}", x)).join("")
}

#[tokio::test]
async fn create_kubernetes_dev_secret() {
    use dialoguer::{theme::ColorfulTheme, console::style, Input};
    use kube::api::PostParams;

    let theme = ColorfulTheme::default();
    println!("{} Generating JWT Secret · {}", style("✔").green().bold(), style("·················").green());

    let secret = BackendSecrets {
        jwt_secret: generate_random_hex::<80>(),
        mongo: MongoSecrets {
            username: Input::with_theme(&theme)
                .with_prompt("MongoDB Username")
                .interact()
                .unwrap(),
            password: Input::with_theme(&theme)
                .with_prompt("MongoDB Password")
                .interact()
                .unwrap(),
            host: Input::with_theme(&theme)
                .with_prompt("MongoDB Host")
                .interact()
                .unwrap(),
        },
    };

    let secret = create_kube_secret_object(secret, SECRET_NAME, DEV_NAMESPACE).await;

    let client = get_client().await;
    let secret_api: Api<Secret> = Api::namespaced(client, DEV_NAMESPACE);

    match secret_api.create(&PostParams::default(),&secret).await {
        Ok(_) => println!("{} Created secret {} in namespace {}",
            style("✔").green().bold(),
            style(SECRET_NAME).green(),
            style(DEV_NAMESPACE).green()
        ),
        Err(kube::Error::Api(kube::core::ErrorResponse { reason, .. })) if reason == "AlreadyExists" => panic!("Secret already exists"),
        Err(e) => panic!("Error getting secret: {:?}", e),
    }
}

#[tokio::test]
async fn can_get_namespace() {

    let namespace = get_namespace().await;

    println!("{}", namespace);
}


