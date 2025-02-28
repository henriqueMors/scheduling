use diesel::prelude::*;
use uuid::Uuid;
use crate::db::Pool;
use crate::models::admin::{Admin, NewAdmin};
use crate::schema::admins; // Tabela `admins` no Diesel

/// 🔹 Adiciona um novo administrador ao banco de dados.
pub fn add_admin(
    conn: &mut PgConnection,
    payload: NewAdmin
) -> Result<Admin, diesel::result::Error> {
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

    Ok(new_admin)  // ✅ Agora retorna `Admin` corretamente
}

/// 🔹 Lista todos os administradores.
pub fn list_admins(conn: &mut PgConnection) -> Result<Vec<Admin>, diesel::result::Error> {
    admins::table.load::<Admin>(conn)
}

/// 🔹 Remove um administrador pelo ID.
pub fn remove_admin(
    conn: &mut PgConnection,
    admin_id: Uuid
) -> Result<usize, diesel::result::Error> {
    diesel::delete(admins::table.filter(admins::id.eq(admin_id)))
        .execute(conn)
}
