use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NewAdmin {
    pub master_id: String,
    pub name: String,
    pub phone: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct AdminResponse {
    pub id: String,
    pub name: String,
    pub phone: String,
}

pub fn add_admin(master_id: String, name: String, phone: String, password_hash: String) -> AdminResponse {
    AdminResponse {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        phone,
    }
}
