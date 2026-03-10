mod queries;
mod routes;

pub use queries::get_user_role; // used in the middelware to attach role info to request
pub use routes::init_routes;