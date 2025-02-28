use diesel::prelude::*;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::db::Pool;
use crate::models::admin::{Admin, NewAdmin};
use crate::schema::admins;

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
    use crate::schema::admins::dsl::*;

    let inserted_admin: Admin = diesel::insert_into(admins)
        .values(&payload)
        .get_result(conn)?;

    Ok(AdminResponse {
        id: inserted_admin.id,
        name: inserted_admin.name,
        phone: inserted_admin.phone,
    })
}

/// Remove um admin pelo ID.
pub fn remove_admin(
    conn: &mut PgConnection,
    admin_id: Uuid
) -> Result<usize, diesel::result::Error> {
    use crate::schema::admins::dsl::*;
    
    diesel::delete(admins.filter(id.eq(admin_id)))
        .execute(conn)
}
