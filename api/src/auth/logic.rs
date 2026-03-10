use actix_web::http::header::HeaderMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::error::ApiError;

#[allow(dead_code)]
pub fn is_expired(expiration_time: &DateTime<Utc>) -> bool {
    *expiration_time < Utc::now()
}

/// Function creates a secure authorization token
/// TODO: Generate secure token
pub fn create_token() -> String {
    Uuid::new_v4().to_string() // Placeholder for actual token generation logic
}

pub fn get_token_from_header(headers: &HeaderMap) -> Result<String, ApiError> {
    headers
        .get("Authorization") // Get auth header
        .ok_or(ApiError::InvalidToken)?
        .to_str()
        .map_err(|_| ApiError::InvalidToken)?
        .strip_prefix("Bearer ") // remove "bearer "
        .ok_or(ApiError::InvalidToken)
        .map(|s| s.to_string())
}