use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable, AsChangeset, Identifiable, Selectable};
use diesel::pg::Pg;
use diesel::sql_types::Uuid as DieselUuid;
use diesel::{AsExpression, FromSqlRow};
use crate::schema::reservations;

#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = DieselUuid)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DieselUuidWrapper(Uuid);

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = reservations)]
#[diesel(check_for_backend(Pg))]
pub struct Reservation {
    pub id: DieselUuidWrapper,
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
