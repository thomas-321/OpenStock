use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct AuthToken {
    pub token_id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub creation_time: NaiveDateTime,
    pub expiration_time: NaiveDateTime,
    pub ip_address: String,
}
