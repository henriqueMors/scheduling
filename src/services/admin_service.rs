use diesel::prelude::*;
use uuid::Uuid;
use crate::db::Pool;
use crate::models::admin::{Admin, NewAdmin};
use crate::schema::admins;

#[derive(Debug, Clone, serde::Serialize)]
pub struct AdminResponse {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
}

/// 🔹 Insere um novo administrador no banco de dados.
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

/// 🔹 Lista todos os administradores.
pub fn list_admins(conn: &mut PgConnection) -> Result<Vec<AdminResponse>, diesel::result::Error> {
    let admins: Vec<Admin> = admins::table.load(conn)?;

    Ok(admins.into_iter().map(|admin| AdminResponse {
        id: admin.id,
        name: admin.name,
        phone: admin.phone,
    }).collect())
}

/// 🔹 Remove um administrador pelo ID.
pub fn remove_admin(
    conn: &mut PgConnection,
    admin_id: Uuid
) -> Result<usize, diesel::result::Error> {
    diesel::delete(admins::table.filter(admins::id.eq(admin_id)))
        .execute(conn)
}
