#[derive(Clone, Debug, thiserror::Error)]
#[error("unexpected null; try decoding as an `Option`")]
pub enum AppError {
    #[error("Could not connect to the api.")]
    ApiUnavailable,

    #[error("Login failed, invalid credentials.")]
    CredentialIncorrect,

    #[error("Error contacting server")]
    ApiClientError,

    #[error("Error reading server response")]
    JsonParseError,

    #[error("Error reading server response")]
    SuccesWithNoContent,

    #[error("You do not have access rights to the requested function")]
    Unauthorized,

    #[error("Login expired.")]
    LoginExpired,

    #[error("Invalid login credentials")]
    InvalidLogin,

    #[error("Missing fields")]
    MissingFields,
}
