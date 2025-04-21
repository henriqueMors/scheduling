use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDate, NaiveTime};
use crate::schema::availabilities;
use diesel::dsl::date;

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = availabilities)]
pub struct Availability {
    pub id: Uuid,
    pub professional_id: Uuid,
    pub date: NaiveDate,      
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,   
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = availabilities)]
pub struct NewAvailability {
    pub professional_id: Uuid,
    pub date: NaiveDate,      
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,   
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = availabilities)]
pub struct UpdateAvailability {
    pub date: Option<NaiveDate>,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
}