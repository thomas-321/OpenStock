use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use serde_json::json;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(get);
    cfg.service(change);
    cfg.service(delete);
}

#[post("")]
async fn create() -> impl Responder {
    HttpResponse::NotImplemented()
        .json(json!({"status": "failed", "message": "Api 'POST /user' not implemented"}))
}

#[get("")]
async fn get() -> impl Responder {
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
