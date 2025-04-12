use diesel::{Queryable, Insertable, Identifiable, Selectable, Associations, AsChangeset};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::models::user::User;
use crate::schema::professionals;
use diesel::sql_types::Array; // Importar o tipo Array

/// 🔹 Estrutura para representar um profissional (para consultas no banco de dados)
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable, Associations, Selectable)]
#[diesel(table_name = professionals)]
#[diesel(belongs_to(User))]  // Relacionamento entre Professional e User
pub struct Professional {
    pub id: Uuid,                // ID do profissional
    pub user_id: Uuid,           // ID do usuário (associado a um User)
    pub bio: Option<String>,     // Biografia (opcional)
    pub specialties: Option<Array<String>>, // Alteração: usando Array<String> para especialidades
    pub created_at: NaiveDateTime, // Data de criação (data e hora)
}

/// 🔹 Estrutura para criar um novo profissional (para inserção no banco)
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = professionals)]
pub struct NewProfessional {
    pub user_id: Uuid,           // ID do usuário associado ao profissional
    pub bio: Option<String>,     // Biografia (opcional)
    pub specialties: Option<Vec<String>>, // Alteração: tornando specialties opcional
}

/// 🔹 Estrutura para atualizar os dados de um profissional (para alteração no banco)
#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = professionals)]
pub struct UpdateProfessional {
    pub bio: Option<String>,     // Biografia (opcional)
    pub specialties: Option<Vec<String>>, // Especialidades (opcional)
}
