#[derive(Debug, thiserror::Error)]
#[error("unexpected null; try decoding as an `Option`")]
pub enum Error {
    #[error("Could not connect to the api.")]
    ApiUnavailable,

    #[error("Login failed, invalid credentials.")]
    CredentialIncorrect,
}