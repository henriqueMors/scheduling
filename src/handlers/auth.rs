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
use crate::schema::users::dsl::*;  // Alterado para usar o DSL
use crate::middleware::auth_middleware::{auth_middleware, Claims};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use tracing::{info, error};

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
#[axum::debug_handler]
pub async fn register_user(
    Extension(pool): Extension<Arc<Pool>>,  // Passando Arc<Pool>
    Json(mut payload): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| {
            error!("Falha ao obter conexão: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    payload.password_hash = hash_password(&payload.password_hash)
        .map_err(|e| {
            error!("Falha no hash: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    let saved_user: User = diesel::insert_into(users)
        .values(&payload)
        .get_result(&mut conn)
        .map_err(|e| {
            error!("Falha no registro: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    info!("Novo usuário registrado: {}", saved_user.id);
    Ok(Json(saved_user))
}

/// Endpoint para login
#[axum::debug_handler]
pub async fn login_user(
    Extension(pool): Extension<Arc<Pool>>,
    Extension(config): Extension<Arc<Config>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| {
            error!("Falha ao obter conexão: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    info!("Tentativa de login: {}", payload.phone);

    // Ajuste: utilizando `first` e limitando a consulta a um único resultado
    let user = users
        .filter(phone.eq(&payload.phone))
        .limit(1)  // Limitando a consulta a apenas um resultado
        .first::<User>(&mut conn)
        .optional()  // Tornando a consulta opcional
        .map_err(|e| {
            error!("Erro na query: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?
        .ok_or_else(|| {
            error!("Telefone não encontrado: {}", payload.phone);
            (StatusCode::UNAUTHORIZED, "Credenciais inválidas".to_string())
        })?;

    if !verify_password(&user.password_hash, &payload.password) {
        error!("Senha incorreta para: {}", payload.phone);
        return Err((StatusCode::UNAUTHORIZED, "Credenciais inválidas".to_string()));
    }

    let token = generate_jwt(&user, &config)
        .map_err(|e| {
            error!("Erro ao gerar token: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    info!("Login bem-sucedido: {}", payload.phone);
    Ok(Json(LoginResponse { token }))
}

/// Endpoint /me
#[axum::debug_handler]
pub async fn me(
    Extension(pool): Extension<Arc<Pool>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<User>, (StatusCode, String)> {
    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| {
            error!("ID inválido no token");
            (StatusCode::BAD_REQUEST, "ID inválido".to_string())
        })?;

    let mut conn = pool.get()
        .map_err(|e| {
            error!("Falha na conexão: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    // Ajuste: utilizando `first` e limitando a consulta a um único resultado
    let user = users
        .filter(id.eq(user_id))
        .limit(1)  // Limitando a consulta a apenas um resultado
        .first::<User>(&mut conn)
        .map_err(|e| {
            error!("Usuário não encontrado: {} - {:?}", user_id, e);
            (StatusCode::NOT_FOUND, "Usuário não encontrado".to_string())
        })?;

    info!("Dados retornados para: {}", user_id);
    Ok(Json(user))
}

/// Rotas de autenticação
pub fn auth_router(pool: Arc<Pool>, config: Arc<Config>) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/me", get(me).layer(middleware::from_fn(auth_middleware)))
        .layer(Extension(pool))
        .layer(Extension(config))
}
