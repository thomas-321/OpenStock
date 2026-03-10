use actix_web::{HttpResponse, ResponseError, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, thiserror::Error, Deserialize, PartialEq)]
#[error("unexpected null; try decoding as an `Option`")]
pub enum ApiError {
    #[error("General database error")]
    DatabaseError,

    #[error("A server error occured")]
    GeneralServerError,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Not authorized")]
    NotAuthorized,

    #[error("Invalid role")]
    InvalidRole,

    #[error("Firstname is too short")]
    FirstnameTooShort,

    #[error("Lastname is too short")]
    LastnameTooShort,

    #[error("Not implemented API call")]
    NotImplemented,

    #[error("Failed to conect to the database")]
    DatabaseConnectionFailed,

    #[error("Database query did not return a row")]
    RowNotFound,

    #[error("Unknown error, catched all reached")]
    UnknownError,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::NotAuthorized => actix_web::http::StatusCode::UNAUTHORIZED,
            ApiError::InvalidCredentials => actix_web::http::StatusCode::UNAUTHORIZED,
            ApiError::InvalidToken => actix_web::http::StatusCode::UNAUTHORIZED,
            ApiError::NotImplemented => actix_web::http::StatusCode::NOT_IMPLEMENTED,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({
            "message": self.to_string(),
        }))
    }
}

impl Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::PoolTimedOut => ApiError::DatabaseConnectionFailed,
            sqlx::Error::RowNotFound => ApiError::RowNotFound,
            sqlx::Error::Database(_) => ApiError::DatabaseConnectionFailed,
            sqlx::Error::Io(_) => ApiError::DatabaseConnectionFailed,
            _ => {
                println!("Unhandled SQLx error: {}", err);
                ApiError::DatabaseError
            }
        }
    }
}