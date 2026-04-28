use models::user::{Role, User};
use std::sync::Arc;

use crate::{error::AppError, util::ApiClient};

/// Function requests the current users data using the token inside ApiClient
pub async fn get_current_user_data(api: Arc<ApiClient>) -> Result<User, AppError> {
    api.send_get_request::<User>("/user/self").await
}

/// Function requests the current users data using the token inside ApiClient
pub async fn get_current_user_role(api: Arc<ApiClient>) -> Result<Role, AppError> {
    api.send_get_request::<Role>("/user/role/self").await
}
