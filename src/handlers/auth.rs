use axum::{
    Router, routing::post, Extension, Json, extract::Header,
    http::StatusCode,
};
use uuid::Uuid;
use diesel::prelude::*;
use crate::db::Pool;
use crate::config::Config;
use crate::services::auth_service::{hash_password, verify_password, generate_jwt};
use crate::models::user::{User};
use crate::schema::users;
use serde::{Serialize, Deserialize};
use headers::Authorization;

/// 🔹 Estrutura para requisição de login
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub phone: String,
    pub password: String,
}

/// 🔹 Estrutura para resposta do login
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

/// 🔹 Endpoint de login
pub async fn login_user(
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Config>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 🔹 Busca o usuário pelo telefone
    let user = users::table
        .filter(users::phone.eq(&payload.phone))
        .first::<User>(&mut conn)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid phone or password".to_string()))?;

    // 🔹 Verifica a senha
    if !verify_password(&user.password_hash, &payload.password) {
        return Err((StatusCode::UNAUTHORIZED, "Invalid phone or password".to_string()));
    }

    // 🔹 Gera um JWT para o usuário autenticado
    let token = generate_jwt(&user, &config)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(LoginResponse { token }))
}

/// 🔹 Define as rotas do módulo `auth`
pub fn router(pool: Pool, config: Config) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user)) // ✅ Adicionando login
        .layer(Extension(pool))
        .layer(Extension(config))
}
