use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize)]
pub struct Reservation {
    pub id: Uuid,
    pub client_id: Uuid,
    pub service: String,
    pub appointment_time: chrono::NaiveDateTime,
    pub status: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::reservations)]
pub struct NewReservation {
    pub client_id: Uuid,
    pub service: String,
    pub appointment_time: chrono::NaiveDateTime,
    pub status: String,
}
