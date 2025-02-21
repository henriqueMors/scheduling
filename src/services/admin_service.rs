use diesel::prelude::*;
use crate::models::user::{User, NewUser};
use crate::schema::users::dsl::*;
use uuid::Uuid;

/// Adiciona um novo administrador secundário.
/// Apenas o administrador master pode adicionar novos administradores.
pub fn add_admin(
    conn: &mut PgConnection,
    master_id: Uuid,
    new_name: String,
    new_phone: String,
    new_password_hash: String
) -> Result<User, String> {
    // Busca o administrador master para verificar a permissão
    let master_user: User = users
        .filter(id.eq(master_id))
        .first(conn)
        .map_err(|_| "Master admin not found.".to_string())?;
    
    if master_user.role != "admin_master" {
        return Err("You do not have permission to add new admins.".to_string());
    }

    // Cria um novo usuário com role "admin"
    let new_user = NewUser {
        name: new_name,
        phone: new_phone,
        password_hash: new_password_hash,
        role: "admin".to_string(),
        sms_verified: false,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
        .map_err(|e| e.to_string())
}

/// Remove um administrador secundário.
/// Apenas o administrador master pode remover administradores.
/// O administrador master não pode ser removido.
pub fn remove_admin(
    conn: &mut PgConnection,
    master_id: Uuid,
    admin_id: Uuid
) -> Result<(), String> {
    let master_user: User = users
        .filter(id.eq(master_id))
        .first(conn)
        .map_err(|_| "Master admin not found.".to_string())?;
    
    if master_user.role != "admin_master" {
        return Err("You do not have permission to remove admins.".to_string());
    }

    // Busca o administrador que se deseja remover
    let admin_to_remove: User = users
        .filter(id.eq(admin_id))
        .first(conn)
        .map_err(|_| "Admin not found.".to_string())?;
    
    if admin_to_remove.role == "admin_master" {
        return Err("Cannot remove master admin.".to_string());
    }

    diesel::delete(users.filter(id.eq(admin_id)))
        .execute(conn)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}
