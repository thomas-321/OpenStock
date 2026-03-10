// external imports
use chrono::{DateTime, Utc};
use uuid::Uuid;

// internal imports
use crate::database;
use crate::error::ApiError;
use crate::db_models::auth::AuthToken;

#[derive(sqlx::FromRow)]
struct UserUuid {
    user_id: Uuid,
}

pub async fn check_credentials(email: &str, password: &str) -> Result<Uuid, ApiError> {
    let db = database::get_pool().await;

    // todo: chang in the future
    let result: Result<Option<UserUuid>, sqlx::Error> = sqlx::query_as(
        r#"SELECT user_id FROM public.users 
           WHERE email = $1 AND password = $2"#,
    )
    .bind(email)
    .bind(password)
    .fetch_optional(&*db)
    .await;

    match result {
        Ok(optional_value) => Ok(optional_value.ok_or(ApiError::NotAuthorized)?.user_id),
        Err(e) => {
            println!("Error checking credentials: {}", e);
            Err(ApiError::from(e))
        }
    }
}

pub async fn get_tokendata_from_db(token: &str) -> Result<AuthToken, ApiError> {
    let db = database::get_pool().await;

    let result: Result<AuthToken, sqlx::Error> = sqlx::query_as(
        r#"SELECT token_id, user_id, token, creation_time, expiration_time, ip_address 
           FROM authentication_tokens WHERE token = $1"#,
    )
    .bind(token)
    .fetch_one(&*db)
    .await;

    match result.map_err(ApiError::from) {
        Ok(value) => Ok(value),
        Err(ApiError::RowNotFound) => Err(ApiError::InvalidToken),
        Err(other) => Err(other),
    }
}

pub async fn add_token_to_db(
    token: &str,
    user_id: &Uuid,
    expiration_time: &DateTime<Utc>,
) -> Result<(), ApiError> {
    let db = database::get_pool().await;

    let token_id = Uuid::new_v4();

    let result = sqlx::query(
        r#"INSERT INTO authentication_tokens (token_id, user_id, token, expiration_time) VALUES
        ($1, $2, $3, $4)"#,
    )
    .bind(token_id)
    .bind(user_id)
    .bind(token)
    .bind(expiration_time)
    .fetch_optional(&*db)
    .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(ApiError::from(e)),
    }
}

pub async fn delete_token(token: &str) -> Result<(), ApiError> {
    let db = database::get_pool().await;

    let result = sqlx::query(r#"DELETE FROM public.authentication_tokens WHERE token = $1"#)
        .bind(token)
        .execute(&*db)
        .await;

    match result {
        Ok(value) => {
            if value.rows_affected() == 0 {
                Err(ApiError::InvalidToken)
            } else {
                Ok(())
            }
        }
        Err(e) => Err(ApiError::from(e)),
    }
}