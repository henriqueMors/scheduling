use axum::{
    Router, routing::{post, get}, Extension, Json, middleware,
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
use crate::middlewares::auth_middleware::{auth_middleware, Claims}; // ✅ Importamos o middleware JWT
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
        user_id: saved_user.id,  
        name: saved_user.name.clone(),
        email: Some(format!("email+{}@exemplo.com", saved_user.id)), 
        phone: saved_user.phone.clone(), 
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
    Extension(claims): Extension<Claims>, // ✅ Agora o middleware garante que `Claims` está presente
) -> Result<Json<User>, (StatusCode, String)> {
    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid user ID format".to_string()))?;

    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 🔹 Busca o usuário pelo ID
    let user = users::table
        .filter(users::id.eq(user_id))
        .first::<User>(&mut conn)
        .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(user))
}

/// 🔹 Define as rotas do módulo `auth`
pub fn router(pool: Pool, config: Arc<Config>) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/me", get(me).layer(middleware::from_fn_with_state(config.clone(), auth_middleware))) // ✅ Middleware antes de `/me`
        .layer(Extension(pool))
        .layer(Extension(config))
}
