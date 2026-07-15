use models::user::{Role, User};
use uuid::Uuid;

use crate::database;
use crate::error::ApiError;

pub async fn get_user_role(user_id: Uuid) -> Result<Role, ApiError> {
    let db = database::get_pool().await;

    let result = sqlx::query_as!(
        Role,
        r#"SELECT * FROM public.roles
           WHERE role_id = (select role_id from users where user_id=$1)"#,
        user_id
    )
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

pub async fn get_user_with_id(user_id: &Uuid) -> Result<User, ApiError> {
    let db = database::get_pool().await;
    sqlx::query_as!(
        User,
        r#"SELECT user_id, role_id, first_name, last_name, status_name FROM public.users
           JOIN user_status_types on users.status = user_status_types.status_id
           WHERE user_id=$1"#,
        user_id
    )
    .fetch_one(&*db)
    .await
    .map_err(ApiError::from)
}
