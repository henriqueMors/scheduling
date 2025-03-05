use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable, AsChangeset};
use crate::schema::reservations;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Reservation {
    pub id: Uuid,
    pub user_id: Uuid,   // 🔹 Relaciona com a tabela `users`
    pub client_id: Uuid, // 🔹 Relaciona com a tabela `clients`
    pub service: String,
    pub appointment_time: NaiveDateTime,
    pub status: String,  // 🔹 Exemplo de status: "pending", "confirmed", "canceled"
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = reservations)]
pub struct NewReservation {
    pub user_id: Uuid,   // 🔹 Identifica o usuário que fez a reserva
    pub client_id: Uuid, // 🔹 Identifica o cliente para o qual a reserva foi feita
    pub service: String,
    pub appointment_time: NaiveDateTime,
    #[serde(default = "default_status")] // 🔹 Define "pending" como padrão
    pub status: String,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = reservations)]
pub struct UpdateReservation {
    pub service: Option<String>,
    pub appointment_time: Option<NaiveDateTime>,
    pub status: Option<String>,
}

// 🔹 Função para definir um status padrão ao criar uma reserva
fn default_status() -> String {
    "pending".to_string()
}
