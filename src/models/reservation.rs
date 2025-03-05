use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable, AsChangeset, Identifiable, Selectable};
use crate::schema::reservations;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = reservations)]
#[diesel(check_for_backend(Pg))] // 🔹 Garante compatibilidade com PostgreSQL
pub struct Reservation {
    pub id: Uuid,
    pub client_id: Uuid,
    pub service: String,
    pub appointment_time: NaiveDateTime,
    pub status: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = reservations)]
pub struct NewReservation {
    pub client_id: Uuid,
    pub service: String,
    pub appointment_time: NaiveDateTime,
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
