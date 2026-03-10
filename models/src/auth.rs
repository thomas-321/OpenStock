use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}