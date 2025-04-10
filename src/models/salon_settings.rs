use chrono::{NaiveDateTime, NaiveTime}; // Importando os tipos de data e hora corretos
use diesel::{Queryable, Insertable, AsChangeset, Identifiable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::salon_settings;

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = salon_settings)]
pub struct SalonSetting {
    pub id: Uuid,
    pub opening_hour: NaiveTime,  // Usando NaiveTime para hora
    pub closing_hour: NaiveTime,  // Usando NaiveTime para hora
    pub working_days: Vec<String>, // Ex: ["monday", "tuesday", "wednesday"]
    pub created_at: NaiveDateTime, // Usando NaiveDateTime para data
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = salon_settings)]
pub struct NewSalonSetting {
    pub opening_hour: NaiveTime,
    pub closing_hour: NaiveTime,
    pub working_days: Vec<String>, // Ex: ["monday", "tuesday", "wednesday"]
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = salon_settings)]
pub struct UpdateSalonSetting {
    pub opening_hour: Option<NaiveTime>,
    pub closing_hour: Option<NaiveTime>,
    pub working_days: Option<Vec<String>>, // Optional para o update
}
