use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::Pool;
use crate::models::admin::{Admin, NewAdmin};
use crate::schema::admins; // Assumindo que há uma tabela chamada `admins`

#[derive(Serialize, Deserialize)]
pub struct AdminResponse {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
}

/// Insere um novo admin no banco de dados.
pub fn add_admin(
    conn: &mut PgConnection,
    payload: NewAdmin
) -> Result<AdminResponse, diesel::result::Error> {
    let new_admin = Admin {
        id: Uuid::new_v4(),
        master_id: payload.master_id,
        name: payload.name,
        phone: payload.phone,
        password_hash: payload.password_hash,
    };

    diesel::insert_into(admins::table)
        .values(&new_admin)
        .execute(conn)?;

    Ok(AdminResponse {
        id: new_admin.id,
        name: new_admin.name,
        phone: new_admin.phone,
    })
}

/// Remove um admin pelo ID.
pub fn remove_admin(
    conn: &mut PgConnection,
    admin_id: Uuid
) -> Result<usize, diesel::result::Error> {
    diesel::delete(admins::table.filter(admins::id.eq(admin_id)))
        .execute(conn)
}
