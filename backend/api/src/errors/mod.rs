

pub mod authentication_error {
    use std::fmt::{Display, Formatter};

    use serde::{Serialize, Deserialize};

    use super::general_errors::GeneralError;


    #[derive(Debug, Serialize, Deserialize)]
    pub enum AuthenticationError {
        InvalidPassword,
        FailedToCreateToken,
        InvalidCredentials,
        InvalidToken,
        FailedToCreateUser(String),
        UserNotFound
    }

    impl Display for AuthenticationError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                AuthenticationError::InvalidPassword => write!(f, "InvalidPassword"),
                AuthenticationError::FailedToCreateToken => write!(f, "FailedToCreateToken"),
                AuthenticationError::InvalidCredentials => write!(f, "InvalidCredentials"),
                AuthenticationError::FailedToCreateUser(err) => write!(f, "FailedToCreateUser{}", err),
                AuthenticationError::UserNotFound => write!(f, "User not found"),
                AuthenticationError::InvalidToken => write!(f, "Invalid token")
            }
        }
    }

    impl std::error::Error for AuthenticationError {}
    impl Into<GeneralError> for AuthenticationError {
        fn into(self) -> GeneralError {
            GeneralError::Authentication(self)
        }
    }
}

pub mod mongo_error {
    use std::fmt::{Display, Formatter};

    use serde::{Serialize, Deserialize};

    use super::general_errors::GeneralError;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct MongoError(pub String);

    impl Display for MongoError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::error::Error for MongoError {}
    impl Into<GeneralError> for MongoError {
        fn into(self) -> GeneralError {
            GeneralError::Mongo(self)
        }
    }
}

pub mod general_errors {
    use std::fmt::{Display, Formatter};

    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub enum GeneralError {
        Mongo(super::mongo_error::MongoError),
        Authentication(super::authentication_error::AuthenticationError),
        InvalidDocument,
        MissingSecret,
        MissingMongoClient,
        MissingNamespace,
        MissingInput(String),
    }

    impl Display for GeneralError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                GeneralError::Mongo(err) => write!(f, "{}", err),
                GeneralError::Authentication(err) => write!(f, "{}", err),
                GeneralError::InvalidDocument => write!(f, "Invalid document"),
                GeneralError::MissingSecret => write!(f, "Missing secret"),
                GeneralError::MissingMongoClient => write!(f, "Missing mongo client"),
                GeneralError::MissingNamespace => write!(f, "Missing namespace"),
                GeneralError::MissingInput(err) => write!(f, "Missing input: {}", err),
            }
        }
    }

    impl std::error::Error for GeneralError {}
}