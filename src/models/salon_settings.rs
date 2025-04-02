use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDateTime};
use crate::schema::salon_settings;

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = salon_settings)]
pub struct SalonSetting {
    pub id: Uuid,
    pub opening_hour: String,  // Ex: "08:00"
    pub closing_hour: String,  // Ex: "18:00"
    pub working_days: Vec<String>, // Ex: ["monday", "tuesday", "wednesday"]
    pub created_at: NaiveDateTime, // Usando NaiveDateTime em vez de String
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = salon_settings)]
pub struct NewSalonSetting {
    pub opening_hour: String,
    pub closing_hour: String,
    pub working_days: Vec<String>, // Ex: ["monday", "tuesday", "wednesday"]
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = salon_settings)]
pub struct UpdateSalonSetting {
    pub opening_hour: Option<String>,
    pub closing_hour: Option<String>,
    pub working_days: Option<Vec<String>>, // Optional para o update
}
