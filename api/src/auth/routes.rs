use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use sqlx::types::Uuid;

use models::auth::LoginPayload;
use crate::auth::get_token_from_header;
use crate::auth::logic::create_token;
use crate::auth::queries::{
    add_token_to_db, check_credentials, delete_token, get_tokendata_from_db,
};
use crate::error::ApiError;

// auth routes
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(logout);
    cfg.service(check);
}

/// receives a username and password
/// returns a token to send requests to this api if both username and password match
/// if username and password do not match an ApiError is returned
#[post("/auth/login")]
async fn login(payload: web::Json<LoginPayload>) -> Result<impl Responder, ApiError> {
    let user_id: Uuid = match check_credentials(&payload.email, &payload.password).await {
        Err(_) => return Err(ApiError::InvalidCredentials),
        Ok(value) => value,
    };

    let token = create_token();

    let experation_time = chrono::Utc::now() + chrono::Duration::days(1);

    return match add_token_to_db(&token, &user_id, &experation_time).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({ "token": token }))),
        Err(e) => Err(e),
    };
}

/// Remove the token in the authorization header from the database
/// returns Ok(()) if the token was found and deleted
/// else returns an ApiError
#[post("/auth/logout")]
async fn logout(request: HttpRequest) -> Result<impl Responder, ApiError> {
    let token = get_token_from_header(request.headers())?;

    return match delete_token(token.as_str()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({"message": "Logged out successfully"}))),
        Err(e) => Err(e),
    };
}

#[get("/auth/check")]
async fn check(request: HttpRequest) -> Result<impl Responder, ApiError> {
    let token = get_token_from_header(request.headers())?;

    get_tokendata_from_db(token.as_str()).await?;

    Ok(HttpResponse::Ok().json(json!({"message": "Api token is valid"})))
}