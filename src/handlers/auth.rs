use axum::{
    Router, routing::post, Extension, Json,
    http::StatusCode,
};
use diesel::prelude::*;
use std::sync::Arc;
use crate::db::Pool;
use crate::config::Config;
use crate::services::auth_service::{hash_password, verify_password, generate_jwt};
use crate::models::user::{User, NewUser};
use crate::schema::users;
use serde::{Serialize, Deserialize};

/// Estrutura para requisição de login
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub phone: String,
    pub password: String,
}

/// Estrutura para resposta do login
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

/// Endpoint para registro de usuário
pub async fn register_user(
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Arc<Config>>,
    Json(mut payload): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Hash da senha
    payload.password_hash = hash_password(&payload.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Insere no banco de dados
    diesel::insert_into(users::table)
        .values(&payload)
        .execute(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Recupera o usuário salvo
    let saved_user = users::table
        .order(users::id.desc())
        .first::<User>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(saved_user))
}

/// Endpoint para login
pub async fn login_user(
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Arc<Config>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = users::table
        .filter(users::phone.eq(&payload.phone))
        .first::<User>(&mut conn)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid phone or password".to_string()))?;

    if !verify_password(&user.password_hash, &payload.password) {
        return Err((StatusCode::UNAUTHORIZED, "Invalid phone or password".to_string()));
    }

    let token = generate_jwt(&user, &config)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(LoginResponse { token }))
}

/// Define as rotas do módulo `auth`
pub fn router(pool: Pool, config: Arc<Config>) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .layer(Extension(pool))
        .layer(Extension(config))
}
