use actix_web::{
    delete, get, patch, post,
    web::{self, ReqData},
    HttpResponse, Responder,
};
use serde_json::json;

use crate::{auth::AuthContext, error::ApiError, user::queries::get_user_with_id};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(get);
    cfg.service(get_self);
    cfg.service(change);
    cfg.service(delete);
}

#[post("")]
async fn create() -> impl Responder {
    HttpResponse::NotImplemented()
        .json(json!({"status": "failed", "message": "Api 'POST /user' not implemented"}))
}

/// Used for a client to get its own user data
/// The auth token used in in the request will be used to match
/// against.
/// returns User if own user data is requested.
#[get("/self")]
async fn get_self(auth: ReqData<AuthContext>) -> Result<impl Responder, ApiError> {
    let user = get_user_with_id(&auth.user_id).await?;
    Ok(HttpResponse::Ok().json(user))
}

/// Used for a client to get the limited data of all users
/// 1. Users role is found using the token
/// 2. The users access rights are checked
/// returns Vec<User> other users info is requested
#[get("")]
async fn get(auth: ReqData<AuthContext>) -> impl Responder {
    //auth.
    HttpResponse::NotImplemented()
        .json(json!({"status": "failed", "message": "Api 'GET /user' not implemented"}))
}

#[patch("")]
async fn change() -> impl Responder {
    HttpResponse::NotImplemented()
        .json(json!({"status": "failed", "message": "Api 'PATCH /user' not implemented"}))
}

#[delete("")]
async fn delete() -> impl Responder {
    HttpResponse::NotImplemented()
        .json(json!({"status": "failed", "message": "Api 'DELETE /user' not implemented"}))
}
