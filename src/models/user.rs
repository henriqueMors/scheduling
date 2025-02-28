// Caso ainda não esteja presente, adicione a linha abaixo no início do seu projeto (por exemplo, em main.rs ou lib.rs)
// #[macro_use] extern crate diesel;
use diesel::prelude::*;
use diesel::Insertable;
use serde::{Serialize, Deserialize};
use crate::schema::users;
use diesel::Queryable;
use diesel::AsChangeset;
use uuid::Uuid;

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub password_hash: String,
    pub role: String,       // "client", "admin" ou "admin_master"
    pub sms_verified: bool,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub phone: String,
    pub password_hash: String,
    pub role: String,
    pub sms_verified: bool,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub password_hash: Option<String>,
    pub role: Option<String>,
    pub sms_verified: Option<bool>,
}
