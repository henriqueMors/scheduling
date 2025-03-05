use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use diesel::sql_types::Uuid as DieselUuid;
use diesel::{AsExpression, FromSqlRow};
use crate::schema::admins;

#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = DieselUuid)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DieselUuidWrapper(Uuid);

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Admin {
    pub id: DieselUuidWrapper,
    pub master_id: Uuid,
    pub name: String,
    pub phone: String,
    pub password_hash: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = admins)]  // ✅ Permite inserção na tabela `admins`
pub struct NewAdmin {
    pub master_id: Uuid,
    pub name: String,
    pub phone: String,
    pub password_hash: String,
}
