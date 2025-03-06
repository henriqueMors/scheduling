use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable, AsChangeset, Identifiable, Selectable};
use diesel::pg::Pg;
use diesel::sql_types::Uuid as DieselUuid;
use diesel::{AsExpression, FromSqlRow};
use crate::schema::clients;

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = clients)]
pub struct Client {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = clients)]
pub struct NewClient {
    pub user_id: Uuid,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = clients)]
pub struct UpdateClient {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}