use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable, AsChangeset, Identifiable, Selectable};
use diesel::pg::Pg;
use diesel::sql_types::Uuid as DieselUuid;
use diesel::{AsExpression, FromSqlRow};
use crate::schema::clients;

#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = DieselUuid)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DieselUuidWrapper(Uuid);

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = clients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Client {
    pub id: DieselUuidWrapper,
    pub user_id: Uuid,  // 🔹 Agora temos `user_id` corretamente vinculado
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = clients)]
pub struct NewClient {
    pub user_id: Uuid,  // 🔹 Precisamos passar um `user_id` ao criar um cliente
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = clients)]
pub struct UpdateClient {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}