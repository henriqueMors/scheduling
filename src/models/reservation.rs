use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::reservations;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = reservations)]
pub struct Reservation {
    pub id: Uuid,
    pub client_id: Uuid,
    pub datetime: chrono::NaiveDateTime,
}