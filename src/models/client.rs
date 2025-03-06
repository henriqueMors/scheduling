use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable, AsChangeset, Identifiable, Selectable};
use diesel::pg::Pg;
use diesel::sql_types::Uuid as DieselUuid;
use diesel::{AsExpression, FromSqlRow};
use crate::schema::clients;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = clients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Client {
    pub id: Uuid,  // 🔹 Alterado para `Uuid` diretamente
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