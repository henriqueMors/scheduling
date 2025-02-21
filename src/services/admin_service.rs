use diesel::prelude::*;
use crate::models::user::{User, NewUser, UpdateUser};
use crate::db::Pool;
use crate::schema::users::dsl::*;
use uuid::Uuid;
use crate::services::auth_service::verify_password; // Caso queira verificar a senha para segurança extra

// Adiciona um novo administrador
pub fn add_admin(conn: &mut PgConnection, master_id: Uuid, name: String, phone: String, password_hash: String) -> Result<User, String> {
    let master_user: User = users.filter(id.eq(master_id)).first(conn).map_err(|_| "Master admin not found.")?;
    
    // Verifica se o usuário atual é realmente o master admin
    if master_user.role != "admin_master" {
        return Err("You do not have permission to add new admins.".into());
    }

    // Cria um novo administrador
    let new_user = NewUser {
        name,
        phone,
        password_hash,
        role: "admin".to_string(),
        sms_verified: false,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
        .map_err(|e| e.to_string())
}

// Remove um administrador
pub fn remove_admin(conn: &mut PgConnection, master_id: Uuid, admin_id: Uuid) -> Result<(), String> {
    let master_user: User = users.filter(id.eq(master_id)).first(conn).map_err(|_| "Master admin not found.")?;
    
    // Verifica se o usuário atual é realmente o master admin
    if master_user.role != "admin_master" {
        return Err("You do not have permission to remove admins.".into());
    }

    // Verifica se o admin a ser removido é o master admin
    let admin_to_remove: User = users.filter(id.eq(admin_id)).first(conn).map_err(|_| "Admin not found.")?;
    if admin_to_remove.role == "admin_master" {
        return Err("Cannot remove master admin.".into());
    }

    // Remove o admin
    diesel::delete(users.filter(id.eq(admin_id)))
        .execute(conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}
