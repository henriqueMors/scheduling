use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable, AsChangeset, Identifiable, Selectable};
use crate::schema::clients;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = clients)]
#[diesel(check_for_backend(Pg))] // 🔹 Garante compatibilidade com PostgreSQL
pub struct Client {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = clients)]
pub struct NewClient {
    pub user_id: Uuid,
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
