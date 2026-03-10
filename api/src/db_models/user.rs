use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Role {
    pub role_id: Uuid,
    pub role_name: String,
    pub create_orders: bool,
    pub create_quotations: bool,
    pub create_purchase_orders: bool,
    pub create_articles: bool,
    pub create_customers: bool,
    pub create_suppliers: bool,
    pub create_users: bool,
    pub create_companys: bool,
}