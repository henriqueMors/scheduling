use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable, AsChangeset, Identifiable, Selectable};
use diesel::pg::Pg;
use crate::schema::reservations;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = reservations)]
#[diesel(check_for_backend(Pg))]
pub struct Reservation {
    pub id: Uuid,  // 🔹 Alterado para `Uuid`
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
