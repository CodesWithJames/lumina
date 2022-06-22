mod logger;
mod authentication;
mod error;
mod not_found;
mod secret;
mod mongo;
mod kubernetes_namespace;
mod cors;

pub use logger::LogMiddleware;
pub use authentication::AuthenticationMiddleware;
pub use error::ErrorMiddleware;
pub use not_found::NotFoundMiddleware;
pub use secret::SecretMiddleware;
pub use mongo::MongoDBMiddleware;
pub use kubernetes_namespace::{NamespaceMiddleware, KubernetesNamespace};
pub use cors::CORSMiddleware;