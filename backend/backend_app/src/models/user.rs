use std::sync::Mutex;




/// We only load the basic info from the db and metadata needed for the user for authentication,
/// logging, and other purposes.
pub struct User {
    id: uuid::Uuid,
    email: String,
}

impl User {

    // /// Authenticates a user from a token and returns the basic user info.
    // /// If the token is invalid, returns a AuthenticationError.
    // pub async fn authenticate_from_token(token: &str) -> Result<User, AuthenticationError> {

    // }
}
