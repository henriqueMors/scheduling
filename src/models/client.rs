use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::{Queryable, Insertable, AsChangeset, Identifiable, Selectable};
use crate::schema::clients;

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = clients)]
#[diesel(check_for_backend(diesel::pg::Pg))] // 🔹 Garante compatibilidade com PostgreSQL
pub struct Client {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = clients)]
pub struct NewClient {
    pub user_id: Uuid,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = clients)]
pub struct UpdateClient {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}
