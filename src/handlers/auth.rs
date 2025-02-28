use axum::{Router, routing::post, Extension, Json, http::StatusCode};
use uuid::Uuid;
use diesel::prelude::*;
use crate::db::Pool;
use crate::services::auth_service::hash_password;
use crate::models::user::{User, NewUser};
use crate::schema::users; // Importa a tabela do banco

/// Endpoint de registro de usuário.
pub async fn register_user(
    Extension(pool): Extension<Pool>,
    Json(mut payload): Json<NewUser>, // Agora aceitamos `NewUser`
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Hash da senha antes de salvar no banco
    payload.password_hash = hash_password(&payload.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Insere no banco de dados
    diesel::insert_into(users::table)
        .values(&payload) // Agora `NewUser` implementa `Insertable`
        .execute(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Recupera o usuário salvo
    let saved_user = users::table
        .order(users::id.desc())
        .first::<User>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(saved_user))
}

/// Define as rotas do módulo `auth`
pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .layer(Extension(pool))
}
