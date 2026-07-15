mod logic;
mod queries;
mod routes;

pub use logic::get_token_from_header;
pub use queries::get_tokendata_from_db;
pub use routes::init_routes;

use models::user::Role;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthContext {
    pub user_id: Uuid,
    pub role: Role,
}
