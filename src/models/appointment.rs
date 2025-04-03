use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::appointments;

/// 🔹 Estrutura para representar um agendamento no banco de dados
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = appointments)]
pub struct Appointment {
    pub id: Uuid,
    pub client_id: Uuid,
    pub professional_id: Uuid,
    pub service_id: Uuid,
    pub appointment_time: NaiveDateTime,
    pub status: String,  // Status do agendamento: "pending", "confirmed", "canceled"
}

/// 🔹 Estrutura para criar um novo agendamento (para inserção no banco)
#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = appointments)]
pub struct NewAppointment {
    pub client_id: Uuid,
    pub professional_id: Uuid,
    pub service_id: Uuid,
    pub appointment_time: NaiveDateTime,
    pub status: String,  // Status inicial (geralmente "pending")
}

/// 🔹 Estrutura para atualizar um agendamento (utilizada no método `update`)
#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = appointments)]
pub struct UpdateAppointment {
    pub appointment_time: Option<NaiveDateTime>, // Permite atualização da data/hora
    pub status: Option<String>,                  // Permite atualização do status (ex: "confirmed", "canceled")
}
