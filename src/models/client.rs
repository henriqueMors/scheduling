use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable, AsChangeset};
use crate::schema::clients;

#[derive(Debug, Queryable, Serialize)]
pub struct Client {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = clients)]
pub struct NewClient {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = clients)]
pub struct UpdateClient {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}
