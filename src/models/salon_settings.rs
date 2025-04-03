use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::salon_settings;

/// 🔹 Estrutura para representar as configurações de um salão no banco de dados
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = salon_settings)]
pub struct SalonSetting {
    pub id: Uuid,                          // ID único da configuração do salão
    pub opening_hour: String,               // Hora de abertura, Ex: "08:00"
    pub closing_hour: String,               // Hora de fechamento, Ex: "18:00"
    pub working_days: Vec<String>,          // Dias da semana que o salão funciona, Ex: ["monday", "tuesday", "wednesday"]
    pub created_at: NaiveDateTime,          // Data de criação da configuração (usando NaiveDateTime)
}

/// 🔹 Estrutura para criar uma nova configuração de salão (para inserção no banco)
#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = salon_settings)]
pub struct NewSalonSetting {
    pub opening_hour: String,               // Hora de abertura, Ex: "08:00"
    pub closing_hour: String,               // Hora de fechamento, Ex: "18:00"
    pub working_days: Vec<String>,          // Dias da semana que o salão funciona, Ex: ["monday", "tuesday", "wednesday"]
}

/// 🔹 Estrutura para atualizar uma configuração de salão existente (para atualização no banco)
#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = salon_settings)]
pub struct UpdateSalonSetting {
    pub opening_hour: Option<String>,       // Hora de abertura (opcional para atualização)
    pub closing_hour: Option<String>,       // Hora de fechamento (opcional para atualização)
    pub working_days: Option<Vec<String>>,  // Dias da semana (opcional para atualização)
}
