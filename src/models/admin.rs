use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::schema::admins; // Importa a tabela do Diesel

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Admin {
    pub id: Uuid,
    pub master_id: String,
    pub name: String,
    pub phone: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = admins)]
pub struct NewAdmin {
    pub master_id: String,
    pub name: String,
    pub phone: String,
    pub password_hash: String,
}
