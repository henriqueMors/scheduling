use axum::{
    Router, routing::{post, get}, Extension, Json, middleware,
    http::StatusCode,
};
use diesel::prelude::*;
use std::sync::Arc;
use crate::db::Pool;
use crate::config::Config;
use crate::services::auth_service::{hash_password, verify_password, generate_jwt};
use crate::models::user::{User, NewUser};
use crate::schema::users;
use crate::middleware::auth_middleware::{auth_middleware, Claims};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use tracing::{info, error}; // ✅ Importando `tracing` para logs

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

/// 🔹 Endpoint para registro de usuário
#[axum::debug_handler]
pub async fn register_user(
    Extension(pool): Extension<Pool>,
    Json(mut payload): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| {
            error!("❌ Falha ao obter conexão com o banco de dados: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    // 🔹 Hash da senha antes de salvar
    payload.password_hash = hash_password(&payload.password_hash)
        .map_err(|e| {
            error!("❌ Falha ao fazer hash da senha: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    // 🔹 Insere o usuário na tabela `users`
    let saved_user: User = diesel::insert_into(users::table)
        .values(&payload)
        .get_result(&mut conn)
        .map_err(|e| {
            error!("❌ Falha ao registrar usuário: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    info!("✅ Novo usuário registrado com ID: {}", saved_user.id);

    Ok(Json(saved_user))
}

/// 🔹 Endpoint para login
#[axum::debug_handler]
pub async fn login_user(
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<Arc<Config>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| {
            error!("❌ Falha ao obter conexão com o banco de dados: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    info!("🔑 Tentativa de login com telefone: {}", payload.phone);

    let user = users::table
        .filter(users::phone.eq(&payload.phone))
        .first::<User>(&mut conn)
        .map_err(|_| {
            error!("❌ Tentativa de login falhou para telefone: {}", payload.phone);
            (StatusCode::UNAUTHORIZED, "Invalid phone or password".to_string())
        })?;

    if !verify_password(&user.password_hash, &payload.password) {
        error!("❌ Senha incorreta para telefone: {}", payload.phone);
        return Err((StatusCode::UNAUTHORIZED, "Invalid phone or password".to_string()));
    }

    let token = generate_jwt(&user, &config)
        .map_err(|e| {
            error!("❌ Falha ao gerar token JWT para telefone: {} - Erro: {:?}", payload.phone, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    info!("✅ Login bem-sucedido para telefone: {}", payload.phone);

    Ok(Json(LoginResponse { token }))
}

/// 🔹 Endpoint `/me`: Retorna os dados do usuário autenticado
#[axum::debug_handler]
pub async fn me(
    Extension(pool): Extension<Pool>,
    Extension(claims): Extension<Claims>, // ✅ Middleware agora garante que `Claims` está presente
) -> Result<Json<User>, (StatusCode, String)> {
    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| {
            error!("❌ ID de usuário inválido no token");
            (StatusCode::BAD_REQUEST, "Invalid user ID format".to_string())
        })?;

    let mut conn = pool.get()
        .map_err(|e| {
            error!("❌ Falha ao obter conexão com o banco de dados: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    let user = users::table
        .filter(users::id.eq(user_id))
        .first::<User>(&mut conn)
        .map_err(|_| {
            error!("❌ Usuário com ID {} não encontrado", user_id);
            (StatusCode::NOT_FOUND, "User not found".to_string())
        })?;

    info!("✅ Dados do usuário retornados para ID: {}", user_id);

    Ok(Json(user))
}

/// 🔹 Define as rotas do módulo `auth`
pub fn router(pool: Pool, config: Arc<Config>) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/me", get(me).layer(middleware::from_fn(auth_middleware))) // ✅ Middleware antes de `/me`
        .layer(Extension(pool))
        .layer(Extension(config))
}
