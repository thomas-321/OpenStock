use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::{Error, HttpMessage};

use super::auth::get_token_from_header;
use super::auth::get_tokendata_from_db;
use super::error::ApiError;
use super::user::get_user_role;

pub async fn check_token(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let token_str = get_token_from_header(req.headers())?;

    let token = get_tokendata_from_db(&token_str).await?;

    if !token_is_valid(&token.token) {
        println!("Received token: {}", token_str);
        return Err(ApiError::InvalidToken.into());
    }

    match get_user_role(token.user_id).await {
        Ok(role) => req.extensions_mut().insert(role),
        Err(e) => return Err(ApiError::GeneralServerError.into()),
    };

    let res = next.call(req).await?;
    Ok(res)
}

// dummy until implemented
fn token_is_valid(_token: &str) -> bool {
    true
}
