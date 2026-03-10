#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use actix_web::middleware::from_fn;
use actix_web::{App, HttpServer, middleware::Logger, web};
use std::env;

use database::init_pool;

// internal modules
mod auth;
mod database;
mod db_models;
mod error;
mod middleware;
mod user;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").ok();

    #[allow(clippy::expect_used)] // db connection error can not be gracefully handled
    let database_url = env::var("DATABASE_URL_POSTGRES").expect("DATABASE_URL must be set");

    #[allow(clippy::expect_used)] // db connection error can not be gracefully handled
    init_pool(&database_url)
        .await
        .expect("Database must be initialized");
    println!("Database successfully initialized.");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    #[allow(clippy::expect_used)]
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            //.service(greet)
            //.service(test)
            .configure(auth::init_routes)
            .service(
                web::scope("/user")
                    .configure(user::init_routes)
                    .wrap(from_fn(middleware::check_token)),
            )
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))
    .expect("Failed to bind server")
    .run()
    .await
}

