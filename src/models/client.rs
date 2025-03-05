use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable, AsChangeset};
use crate::schema::clients;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Client {
    pub id: Uuid,
    pub user_id: Uuid,  // 🔹 Relaciona o cliente ao usuário correspondente
    pub name: String,
    pub email: Option<String>, // 🔹 Agora é opcional para maior flexibilidade
    pub phone: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = clients)]
pub struct NewClient {
    pub user_id: Uuid,  // 🔹 Sempre vinculado ao usuário correspondente
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = clients)]
pub struct UpdateClient {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}
