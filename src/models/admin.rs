use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::admins; // Assumindo que há uma tabela chamada `admins`

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "admins"]
pub struct Admin {
    pub id: Uuid,
    pub master_id: String,
    pub name: String,
    pub phone: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "admins"]
pub struct NewAdmin {
    pub master_id: String,
    pub name: String,
    pub phone: String,
    pub password_hash: String,
}
