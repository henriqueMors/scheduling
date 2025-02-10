use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::schema::clients;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Client {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = clients)]
pub struct NewClient {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
}