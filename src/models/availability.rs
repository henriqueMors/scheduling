use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDate, NaiveTime};
use crate::schema::availabilities;

/// 🔹 Estrutura para representar um horário disponível (para consultas no banco de dados)
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = availabilities)]
pub struct Availability {
    pub id: Uuid,
    pub professional_id: Uuid,
    pub date: NaiveDate,       // Data do horário disponível
    pub start_time: NaiveTime, // Hora de início do horário
    pub end_time: NaiveTime,   // Hora de término do horário
}

/// 🔹 Estrutura para criar um novo horário disponível (para inserção no banco)
#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = availabilities)]
pub struct NewAvailability {
    pub professional_id: Uuid, // ID do profissional
    pub date: NaiveDate,       // Data do horário disponível
    pub start_time: NaiveTime, // Hora de início do horário
    pub end_time: NaiveTime,   // Hora de término do horário
}

/// 🔹 Estrutura para atualizar um horário disponível (usada na rota de atualização)
#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = availabilities)]
pub struct UpdateAvailability {
    pub date: Option<NaiveDate>,       // Data a ser atualizada (opcional)
    pub start_time: Option<NaiveTime>, // Hora de início a ser atualizada (opcional)
    pub end_time: Option<NaiveTime>,   // Hora de término a ser atualizada (opcional)
}
