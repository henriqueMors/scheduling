use axum::{
    Router, routing::{post, get}, Extension, Json,
    http::StatusCode,
};
use diesel::prelude::*;
use std::sync::Arc;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::db::Pool;
use crate::config::Config;
use crate::services::auth_service::{hash_password, verify_password, generate_jwt};
use crate::models::user::{User, NewUser};
use crate::models::client::NewClient;
use crate::schema::{users, clients};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

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

/// 🔹 Estrutura para os claims do JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,  // ID do usuário
    pub exp: usize,   // Expiração do token (timestamp UNIX)
    pub role: String, // Papel do usuário (client, admin, admin_master)
}

/// 🔹 Endpoint para registro de usuário (também cria `Client`)
#[axum::debug_handler]
pub async fn register_user(
    Extension(pool): Extension<Pool>,
    Json(mut payload): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 🔹 Hash da senha antes de salvar
    payload.password_hash = hash_password(&payload.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 🔹 Insere o usuário na tabela `users` e obtém o usuário salvo
    let saved_user: User = diesel::insert_into(users::table)
        .values(&payload)
        .get_result(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 🔹 Cria um `Client` automaticamente vinculado ao `User`
    let new_client = NewClient {
        user_id: saved_user.id.0,  // 🔹 Pegamos o `Uuid` dentro de `DieselUuidWrapper`
        name: saved_user.name.clone(),
        email: Some(format!("email+{}@exemplo.com", saved_user.id.0)), // ✅ Email fictício
        phone: Some(saved_user.phone.clone()), // ✅ Corrigido
    };

    // 🔹 Insere o `Client` no banco de dados
    diesel::insert_into(clients::table)
        .values(&new_client)
        .execute(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

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
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

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

/// 🔹 Endpoint `/me`: Retorna os dados do usuário autenticado
#[axum::debug_handler]
pub async fn me(
    Extension(pool): Extension<Pool>,
    Extension(claims): Extension<Claims>, // ✅ Obtém `Claims` injetado pelo middleware
) -> Result<Json<User>, (StatusCode, String)> {
    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid user ID format".to_string()))?;

    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 🔹 Busca o usuário pelo ID
    let user = users::table
        .filter(users::id.eq(user_id)) // ✅ Diesel agora aceita diretamente `Uuid`
        .first::<User>(&mut conn)
        .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(user))
}

/// 🔹 Define as rotas do módulo `auth`
pub fn router(pool: Pool, config: Arc<Config>) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/me", get(me))
        .layer(Extension(pool))
        .layer(Extension(config))
}
