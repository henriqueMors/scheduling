use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::admins;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Admin {
    pub id: Uuid,
    pub master_id: Uuid,
    pub name: String,
    pub phone: String,
    pub password_hash: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = admins)]  // ✅ Permite inserção na tabela `admins`
pub struct NewAdmin {
    pub master_id: Uuid,
    pub name: String,
    pub phone: String,
    pub password_hash: String,
}
