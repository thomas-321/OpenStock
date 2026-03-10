// external imports
use uuid::Uuid;

// internal imports
use crate::database;
use crate::db_models::user::Role;
use crate::error::ApiError;

pub async fn get_user_role(user_id: Uuid) -> Result<Role, ApiError> {
    let db = database::get_pool().await;

    let result: Result<Role, sqlx::Error> = sqlx::query_as(
        r#"SELECT * FROM public.roles
           WHERE role_id = (select role_id from users where user_id=$1)"#,
    )
    .bind(user_id)
    .fetch_one(&*db)
    .await;

    match result {
        Ok(value) => Ok(value),
        Err(e) => {
            println!("Error checking credentials: {}", e);
            Err(ApiError::from(e))
        }
    }
}
