use diesel::{Insertable, Queryable, AsChangeset, Identifiable, Selectable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::users;

/// 🔹 Estrutura que representa o usuário no banco de dados
#[derive(Debug, Queryable, Serialize, Deserialize, Selectable, Identifiable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,                     // ID único do usuário
    pub name: String,                 // Nome do usuário
    pub email: String,                // Email do usuário (deve ser único)
    pub password_hash: String,        // Hash da senha (nunca armazenar senha em texto plano)
    pub role: String,                 // Função do usuário: "cliente", "admin", "profissional"
    pub created_at: NaiveDateTime,    // Data de criação do usuário
}

/// 🔹 Estrutura para criar um novo usuário
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,                 // Nome do usuário
    pub email: String,                // Email do usuário
    pub password_hash: String,        // Hash da senha
    pub role: String,                 // Função do usuário (deve ser "cliente", "admin", "profissional")
}

/// 🔹 Estrutura para atualizar os dados de um usuário existente
#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub name: Option<String>,         // Nome do usuário (opcional para atualização)
    pub email: Option<String>,        // Email do usuário (opcional para atualização)
    pub password_hash: Option<String>, // Hash da senha (opcional para atualização)
    pub role: Option<String>,         // Função do usuário (opcional para atualização)
}