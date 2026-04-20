use crate::error::AppError;
use crate::state::ApiClient;
use models::auth::{LoginPayload, LoginResponse};

pub async fn login(
    api: ApiClient,
    email: Option<String>,
    password: Option<String>,
) -> Result<LoginResponse, AppError> {
    let email: String = email.ok_or(AppError::MissingFields)?;
    let password: String = password.ok_or(AppError::MissingFields)?;

    let result = api
        .auth_request(
            api.client
                .post(format!("{}/projects", api.base_url))
                .json(&LoginPayload { email, password }),
        )
        .send()
        .await;

    let result = result.unwrap_or(Err(AppError::ApiClientError)?);

    result
        .json::<LoginResponse>()
        .await
        .or(Err(AppError::JsonParseError))
}
