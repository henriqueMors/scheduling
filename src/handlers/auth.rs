use axum::{Router, routing::post, Extension, Json, http::StatusCode};
use uuid::Uuid;
use diesel::prelude::*;
use crate::db::Pool;
use crate::services::auth_service::hash_password;
use crate::models::user::{User, NewUser};
use crate::schema::users; // Tabela do banco de dados

/// Endpoint de registro de usuário.
pub async fn register_user(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Hash da senha
    let hashed_password = hash_password(&payload.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let new_user = User {
        id: Uuid::new_v4(),
        name: payload.name,
        phone: payload.phone,
        password_hash: hashed_password, // Agora armazenamos a senha corretamente
        role: payload.role,
        sms_verified: payload.sms_verified,
    };

    // Insere o usuário no banco de dados
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(new_user)) // ✅ Retornamos o usuário cadastrado
}

/// Define as rotas do módulo `auth`
pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .layer(Extension(pool))
}
