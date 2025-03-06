use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use diesel::sql_types::Uuid as DieselUuid;
use diesel::{AsExpression, FromSqlRow};
use crate::schema::admins;

#[derive(AsExpression, FromSqlRow, Debug, Clone, Serialize, Deserialize)]
#[diesel(sql_type = DieselUuid)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DieselUuidWrapper(Uuid);

impl From<Uuid> for DieselUuidWrapper {
    fn from(uuid: Uuid) -> Self {
        DieselUuidWrapper(uuid)
    }
}

impl From<DieselUuidWrapper> for Uuid {
    fn from(wrapper: DieselUuidWrapper) -> Self {
        wrapper.0
    }
}

/// 🔹 Modelo do Administrador
#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = admins)]
pub struct Admin {
    pub id: Uuid,  // 🔹 Agora usamos `Uuid` diretamente para evitar erros no Diesel
    pub master_id: Uuid,
    pub name: String,
    pub phone: String,
    pub password_hash: String,
}

/// 🔹 Estrutura para criação de um novo Administrador
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = admins)]
pub struct NewAdmin {
    pub master_id: Uuid,
    pub name: String,
    pub phone: String,
    pub password_hash: String,
}
