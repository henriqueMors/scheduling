use diesel::prelude::*;
use uuid::Uuid;
use crate::db::Pool;
use crate::models::admin::{Admin, NewAdmin};
use crate::schema::admins; // Referência à tabela Diesel

/// 🔹 Adiciona um novo administrador ao banco de dados.
pub fn add_admin(
    conn: &mut PgConnection,
    payload: NewAdmin
) -> Result<Admin, diesel::result::Error> {
    diesel::insert_into(admins::table)
        .values(&payload)
        .execute(conn)?;

    // ✅ Buscar o admin inserido para retornar com ID gerado
    admins::table
        .order(admins::id.desc())  // Pega o mais recente
        .first::<Admin>(conn)
}

/// 🔹 Lista todos os administradores do sistema.
pub fn list_admins(conn: &mut PgConnection) -> Result<Vec<Admin>, diesel::result::Error> {
    admins::table.load::<Admin>(conn)  // ✅ Corrige o erro "função não encontrada"
}

/// 🔹 Remove um administrador pelo ID.
pub fn remove_admin(conn: &mut PgConnection, admin_id: Uuid) -> Result<usize, diesel::result::Error> {
    diesel::delete(admins::table.filter(admins::id.eq(admin_id)))
        .execute(conn)
}
