use diesel::{Queryable, Insertable, AsChangeset};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveTime, NaiveDateTime};
use crate::schema::salon_settings;

/// Estrutura que representa as configurações do salão
#[derive(Debug, Queryable, Serialize, Deserialize)]
#[diesel(table_name = salon_settings)]
pub struct SalonSetting {
    pub id: Uuid,
    pub opening_hour: NaiveTime,  // Alterado para NaiveTime
    pub closing_hour: NaiveTime,  // Alterado para NaiveTime
    pub working_days: Vec<String>, // Ex: ["monday", "tuesday", "wednesday"]
    pub created_at: NaiveDateTime, // Usando NaiveDateTime para a data de criação
}

/// Estrutura para criar uma nova configuração do salão
#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = salon_settings)]
pub struct NewSalonSetting {
    pub opening_hour: NaiveTime,   // Alterado para NaiveTime
    pub closing_hour: NaiveTime,   // Alterado para NaiveTime
    pub working_days: Vec<String>, // Ex: ["monday", "tuesday", "wednesday"]
}

/// Estrutura para atualizar a configuração do salão
#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = salon_settings)]
pub struct UpdateSalonSetting {
    pub opening_hour: Option<NaiveTime>,   // Alterado para NaiveTime
    pub closing_hour: Option<NaiveTime>,   // Alterado para NaiveTime
    pub working_days: Option<Vec<String>>, // Opcional para atualização
}
