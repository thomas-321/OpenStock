use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub status_name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
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
