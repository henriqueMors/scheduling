use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::appointments;

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = appointments)]
pub struct Appointment {
    pub id: Uuid,
    pub client_id: Uuid,
    pub professional_id: Uuid,
    pub service_id: Uuid,
    pub appointment_time: NaiveDateTime,
    pub status: String,  // Ex: "pending", "confirmed", "canceled"
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = appointments)]
pub struct NewAppointment {
    pub client_id: Uuid,
    pub professional_id: Uuid,
    pub service_id: Uuid,
    pub appointment_time: NaiveDateTime,
    pub status: String,  // "pending"
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = appointments)]
pub struct UpdateAppointment {
    pub appointment_time: Option<NaiveDateTime>,
    pub status: Option<String>,
}
