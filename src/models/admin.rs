use serde::{Serialize, Deserialize};
use uuid::Uuid;
use diesel::prelude::*;
use crate::schema::admins;

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "admins"]
pub struct NewAdmin {  
    pub master_id: Uuid,  // 🔥 Certifique-se que é `Uuid`, não `String`
    pub name: String,
    pub phone: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Admin {
    pub id: Uuid,
    pub master_id: Uuid,
    pub name: String,
    pub phone: String,
    pub password_hash: String,
}
