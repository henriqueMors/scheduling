use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable, AsChangeset, Identifiable, Selectable};
use diesel::pg::Pg;
use crate::schema::reservations;

/// 🔹 Estrutura para representar uma reserva no banco de dados
#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = reservations)]
#[diesel(check_for_backend(Pg))]  // Garante que este código será validado apenas no backend PostgreSQL
pub struct Reservation {
    pub id: Uuid,                  // ID único da reserva
    pub user_id: Uuid,             // ID do usuário que fez a reserva
    pub service: String,           // Serviço que foi reservado
    pub appointment_time: NaiveDateTime, // Data e hora do agendamento
    pub status: String,            // Status da reserva (ex: "pending", "confirmed", "canceled")
}

/// 🔹 Estrutura para criar uma nova reserva (para inserção no banco de dados)
#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = reservations)]
pub struct NewReservation {
    pub user_id: Uuid,             // ID do usuário que fez a reserva
    pub service: String,           // Serviço que foi reservado
    pub appointment_time: NaiveDateTime, // Data e hora do agendamento
    pub status: String,            // Status inicial da reserva (geralmente "pending")
}

/// 🔹 Estrutura para atualizar uma reserva existente (para atualização no banco de dados)
#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = reservations)]
pub struct UpdateReservation {
    pub service: Option<String>,          // Serviço que foi reservado (opcional, pode ser atualizado)
    pub appointment_time: Option<NaiveDateTime>, // Data e hora do agendamento (opcional)
    pub status: Option<String>,           // Status da reserva (opcional)
}
