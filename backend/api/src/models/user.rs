use std::{time::SystemTime, sync::Arc};

use mongodb::{Client as MongoClient, IndexModel, options::IndexOptions, error::{ErrorKind, WriteFailure, WriteError}};

use castle_api::types::State;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use mongodb::{bson::{oid::ObjectId, doc}, Collection, options::FindOneOptions};
use secrets::BackendSecrets;
use serde::{Deserialize, Serialize};

use crate::{
    errors::{
        authentication_error::AuthenticationError, general_errors::GeneralError,
        mongo_error::MongoError,
    },
    middleware::KubernetesNamespace,
};

/// We only load the basic info from the db and metadata needed for the user for authentication,
/// logging, and other purposes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    _id: ObjectId,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenData {
    user_id: ObjectId,
    created: SystemTime,
}

impl User {
    /// Authenticates a user from a token and returns the basic user info.
    /// If the token is invalid, returns a AuthenticationError.
    ///
    /// The token is a JWT token that is signed with a secret key.
    pub async fn authenticate_from_token(
        token: &str,
        secret: &str,
    ) -> Result<User, AuthenticationError> {
        // We want to use a "sliding window" token so there is no direct expiry time.
        // We use the database to store the "last used" time of the token.
        // This means if a user constantly uses the same token they will not be logged out.

        let mut validation = Validation::new(Algorithm::HS256);
        // remove default required_spec_claims
        validation.set_required_spec_claims::<&str>(&[]);
        // disable expiry valiation
        validation.validate_exp = false;

        match jsonwebtoken::decode::<TokenData>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        ) {
            Ok(decoded) => Ok(User {
                _id: decoded.claims.user_id,
                email: "albert@framework.tools".to_string(),
            }),
            Err(e) => {
                tracing::error!("{}", e);
                Err(AuthenticationError::InvalidToken)
            }
        }
    }

    pub async fn login(
        state: &State,
        email: &str,
        password: &str,
    ) -> Result<String, GeneralError> {
        let secrets = state.borrow::<Arc<BackendSecrets>>();

        let client = &state.borrow::<MongoClient>();
        let collection: Collection<ProjectedUser> = client.database(state.borrow::<KubernetesNamespace>().as_ref()).collection("users");
        let find_options = FindOneOptions::builder()
            .projection(doc! {
                "_id": 1,
                "email": 1,
                "password": 1,
            }).build();

        #[derive(Serialize, Deserialize)]
        struct ProjectedUser {
            _id: ObjectId,
            email: String,
            password: String,
        }

        let user = collection.find_one(doc! { "email": email }, find_options)
            .await
            .map_err(|err| MongoError(err.to_string()).into())?;

        tracing::info!("Login attempt by: {}", email);
        match user {
            Some(user) => {
                // check password
                if let Err(e) = bcrypt::verify(password, &user.password) {
                    tracing::error!("Login attempt with password mismatch {}", e);
                    return Err(AuthenticationError::InvalidPassword.into());
                }
                tracing::info!("Login Success: {}", email);
                let token_data = TokenData {
                    user_id: user._id,
                    created: SystemTime::now(),
                };

                match jsonwebtoken::encode(
                    &jsonwebtoken::Header::default(),
                    &token_data,
                    &EncodingKey::from_secret(secrets.jwt_secret.as_bytes()),
                ) {
                    Ok(token) => Ok(token),
                    Err(_) => Err(AuthenticationError::FailedToCreateToken.into()),
                }
            }
            None => {
                tracing::info!("User Not Found: {}", email);
                Err(AuthenticationError::UserNotFound.into())
            }
        }
    }

    pub async fn create_user(
        state: &State,
        email: &str,
        password: &str,
        first_name: &str,
        last_name: &str,
    ) -> Result<(), anyhow::Error> {
        let client = state.borrow::<MongoClient>();
        let collection: Collection<serde_json::Value> = client.database(state.borrow::<KubernetesNamespace>().as_ref()).collection("users");

        collection.create_index(IndexModel::builder()
            .keys(doc! { "email": 1 })
            .options(Some(
                IndexOptions::builder()
                    .unique(true)
                    .build()
            ))
            .build(), None).await
            .map_err(|e| anyhow::anyhow!{"Failed to create user email index: {}", e})?;

        let user = serde_json::json!({
            "email": email,
            "password": bcrypt::hash(password, 12)
                .map_err(|e| anyhow::anyhow!{"Failed to hash user password"})?,
            "first_name": first_name,
            "last_name": last_name
        });

        collection
            .insert_one(user, None)
            .await
            .map_err(|e| match *e.kind {
                ErrorKind::Write(WriteFailure::WriteError(WriteError {
                    code: 11000, // duplicate key
                    ..
                })) => anyhow::anyhow!{"User already exists"},
                _ => anyhow::anyhow!{"Failed to create user: {}", e}
            })?;

        Ok(())
    }
}