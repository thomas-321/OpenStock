use lazy_static::lazy_static;
use sqlx::{Error, PgPool};
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static! {
    static ref POOL: Arc<Mutex<Option<PgPool>>> = Arc::new(Mutex::new(None));
}

// Initialize the database pool
pub async fn init_pool(database_url: &str) -> Result<(), Error> {
    let pool = PgPool::connect(database_url).await?;
    let mut p = POOL.lock().await;
    *p = Some(pool);
    Ok(())
}

// Access the database pool
pub async fn get_pool() -> Arc<PgPool> {
    let pool = POOL.lock().await;
    if let Some(pool) = &*pool {
        Arc::new(pool.clone())
    } else {
        panic!("Database pool not initialized");
    }
}