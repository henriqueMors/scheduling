use diesel::prelude::*;
use diesel::sql_types::Uuid as DieselUuid;
use diesel::{AsExpression, FromSqlRow};
use diesel::{Insertable, Queryable, AsChangeset};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::users;

#[derive(Debug, Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,  // 🔹 Alterado para `Uuid`
    pub name: String,
    pub phone: String,
    pub password_hash: String,
    pub role: String,
    pub sms_verified: bool,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
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
