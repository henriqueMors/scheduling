use diesel::{Insertable, Queryable, AsChangeset, Identifiable, Selectable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::users;

#[derive(Debug, Queryable, Serialize, Deserialize, Selectable, Identifiable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub password_hash: String,
    pub role: String,
    pub sms_verified: bool,
    pub created_at: NaiveDateTime,
}



/// 🔹 Estrutura para criar um novo usuário
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub phone: String,
    pub password_hash: String,
    pub role: String,
}


/// 🔹 Estrutura para atualizar os dados de um usuário existente
#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub password_hash: Option<String>,
    pub role: Option<String>,
}
