use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDate, NaiveTime}; // Importando NaiveDate e NaiveTime corretamente
use crate::schema::availabilities;

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = availabilities)]
pub struct Availability {
    pub id: Uuid,
    pub professional_id: Uuid,
    pub date: NaiveDate,            // Usando NaiveDate para a data
    pub start_time: NaiveTime,      // Usando NaiveTime para o horário de início
    pub end_time: NaiveTime,        // Usando NaiveTime para o horário de término
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = availabilities)]
pub struct NewAvailability {
    pub professional_id: Uuid,
    pub date: NaiveDate,            // Usando NaiveDate para a data
    pub start_time: NaiveTime,      // Usando NaiveTime para o horário de início
    pub end_time: NaiveTime,        // Usando NaiveTime para o horário de término
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = availabilities)]
pub struct UpdateAvailability {
    pub date: Option<NaiveDate>,    // Usando NaiveDate como tipo opcional
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
}
