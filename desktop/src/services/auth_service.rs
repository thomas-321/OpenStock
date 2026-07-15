use reqwest::StatusCode;
use std::sync::Arc;

use models::auth::{LoginPayload, LoginResponse};

use crate::error::AppError;
use crate::util::ApiClient;

pub async fn login(
    api: Arc<ApiClient>,
    email: Option<String>,
    password: Option<String>,
) -> Result<LoginResponse, AppError> {
    let email: String = email.ok_or(AppError::MissingFields)?;
    let password: String = password.ok_or(AppError::MissingFields)?;

    let result = api
        .auth_request(
            api.client
                .post(format!("{}/auth/login", api.base_url))
                .json(&LoginPayload { email, password }),
        )
        .send()
        .await;

    match result {
        Ok(value) => {
            if value.status() != StatusCode::OK {
                Err(AppError::InvalidLogin)?;
            }

            value
                .json::<LoginResponse>()
                .await
                .or(Err(AppError::JsonParseError))
        }
        Err(e) => {
            println!("{}", e);
            Err(AppError::ApiClientError)?
        }
    }

    //let result = result.unwrap_or(Err(AppError::ApiClientError)?);
    //
    //if result.status() != StatusCode::OK {
    //    Err(AppError::InvalidLogin)?;
    //}
    //
    //result
    //    .json::<LoginResponse>()
    //    .await
    //    .or(Err(AppError::JsonParseError))
}
