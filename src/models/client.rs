use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::clients;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = clients)]
pub struct Client {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
}
