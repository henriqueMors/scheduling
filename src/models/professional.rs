use diesel::{Queryable, Insertable, Identifiable, Selectable, Associations, AsChangeset};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::models::user::User;
use crate::schema::professionals;
use diesel::sql_types::{Text, Array, Nullable};

/// 🔹 Estrutura para representar um profissional
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable, Associations, Selectable)]
#[diesel(table_name = professionals)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]  // Adicione esta linha
pub struct Professional {
    pub id: Uuid,
    pub user_id: Uuid,
    pub bio: Option<String>,
    pub specialties: Option<Vec<Option<String>>>,  // Alterado para Option<Vec<Option<String>>>
    pub created_at: NaiveDateTime,
}

/// 🔹 Estrutura para criar um novo profissional
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = professionals)]
pub struct NewProfessional {
    pub user_id: Uuid,
    pub bio: Option<String>,
    pub specialties: Option<Vec<Option<String>>>,  // Alterado para Option<Vec<Option<String>>>
}

/// 🔹 Estrutura para atualizar os dados de um profissional
#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = professionals)]
pub struct UpdateProfessional {
    pub bio: Option<String>,
    pub specialties: Option<Vec<Option<String>>>,  // Alterado para Option<Vec<Option<String>>>
}