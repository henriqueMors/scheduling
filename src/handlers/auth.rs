use axum::{Router, routing::post, Extension, Json, http::StatusCode};
use uuid::Uuid;
use diesel::prelude::*;
use crate::db::Pool;
use crate::services::auth_service::{hash_password, verify_password, generate_jwt};
use crate::models::user::{User, NewUser, LoginRequest, LoginResponse};
use crate::schema::users;
use crate::config::Config;

/// 🔹 Endpoint de registro de usuário.
pub async fn register_user(
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Config>,
    Json(mut payload): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 🔹 Hash da senha
    payload.password_hash = hash_password(&payload.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 🔹 Inserção no banco
    diesel::insert_into(users::table)
        .values(&payload)
        .execute(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 🔹 Retorna o usuário cadastrado
    let saved_user = users::table
        .order(users::id.desc())
        .first::<User>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(saved_user))
}

/// 🔹 Endpoint de login
pub async fn login_user(
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Config>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 🔹 Busca usuário pelo telefone
    let user = users::table
        .filter(users::phone.eq(&payload.phone))
        .first::<User>(&mut conn)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid phone or password".to_string()))?;

    // 🔹 Verifica a senha
    if !verify_password(&user.password_hash, &payload.password) {
        return Err((StatusCode::UNAUTHORIZED, "Invalid phone or password".to_string()));
    }

    // 🔹 Gera token JWT
    let token = generate_jwt(&user, &config)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(LoginResponse {
        token,
        user_id: user.id,
        role: user.role,
    }))
}

/// 🔹 Define as rotas do módulo `auth`
pub fn router(pool: Pool, config: Config) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user)) // ✅ Adicionado login
        .layer(Extension(pool))
        .layer(Extension(config))
}
