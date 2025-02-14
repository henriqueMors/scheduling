use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::reservations)]
pub struct Reservation {
    pub id: Uuid,
    pub client_id: Uuid,
    pub service: String,
    pub appointment_time: NaiveDateTime,
    pub status: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::reservations)]
pub struct NewReservation {
    pub id: Uuid,
    pub client_id: Uuid,
    pub service: String,
    pub appointment_time: NaiveDateTime,
    pub status: String,
}
