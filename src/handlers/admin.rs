use axum::{
    extract::{Extension, Json, TypedHeader},
    http::StatusCode,
    Router,
};
use headers::{Authorization, Bearer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::prelude::*;
use crate::db::Pool;
use crate::services::admin_service::{add_admin, remove_admin};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

// Define a estrutura das claims que esperamos no token JWT.
// Aqui, 'sub' contém o ID do usuário (admin master) e 'exp' a expiração.
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Deserialize)]
pub struct AddAdminRequest {
    pub name: String,
    pub phone: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RemoveAdminRequest {
    pub admin_id: Uuid,
}

#[derive(Serialize)]
pub struct AdminResponse {
    pub message: String,
}

/// Endpoint para adicionar um novo administrador.
/// Esse endpoint extrai o token JWT do header e verifica se o usuário autenticado é o admin master.
pub async fn add_admin_handler(
    // Extrai o token JWT do header Authorization
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Extension(pool): Extension<Pool>,
    Json(payload): Json<AddAdminRequest>,
) -> Result<Json<AdminResponse>, (StatusCode, String)> {
    // Decodifica o token JWT usando a mesma chave secreta utilizada na geração (secret_key)
    let token_data = decode::<Claims>(
        bearer.token(),
        &DecodingKey::from_secret("secret_key".as_ref()),
        &Validation::new(Algorithm::HS256),
    ).map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    // Extrai o master_id a partir da claim 'sub'
    let master_id = Uuid::parse_str(&token_data.claims.sub)
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    // Obtém uma conexão do pool e busca o usuário pelo ID para confirmar o role
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let master_user: crate::models::user::User = crate::schema::users::dsl::users
        .filter(crate::schema::users::dsl::id.eq(master_id))
        .first(&mut conn)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Admin master not found".into()))?;

    // Verifica se o usuário autenticado possui o role "admin_master"
    if master_user.role != "admin_master" {
        return Err((StatusCode::FORBIDDEN, "You do not have permission to add new admins.".into()));
    }

    // Gera o hash da senha para o novo administrador usando o serviço de autenticação
    let password_hash = crate::services::auth_service::hash_password(&payload.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Chama a função de serviço para adicionar o novo admin
    match add_admin(&mut conn, master_id, payload.name.clone(), payload.phone.clone(), password_hash) {
        Ok(user) => Ok(Json(AdminResponse {
            message: format!("Administrador {} adicionado com sucesso.", user.name),
        })),
        Err(err) => Err((StatusCode::FORBIDDEN, err)),
    }
}

/// Endpoint para remover um administrador.
/// Extrai o token JWT para verificar se o usuário autenticado é o admin master.
pub async fn remove_admin_handler(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Extension(pool): Extension<Pool>,
    Json(payload): Json<RemoveAdminRequest>,
) -> Result<Json<AdminResponse>, (StatusCode, String)> {
    let token_data = decode::<Claims>(
        bearer.token(),
        &DecodingKey::from_secret("secret_key".as_ref()),
        &Validation::new(Algorithm::HS256),
    ).map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    let master_id = Uuid::parse_str(&token_data.claims.sub)
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let master_user: crate::models::user::User = crate::schema::users::dsl::users
        .filter(crate::schema::users::dsl::id.eq(master_id))
        .first(&mut conn)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Admin master not found".into()))?;

    if master_user.role != "admin_master" {
        return Err((StatusCode::FORBIDDEN, "You do not have permission to remove admins.".into()));
    }

    match remove_admin(&mut conn, master_id, payload.admin_id) {
        Ok(_) => Ok(Json(AdminResponse {
            message: "Administrador removido com sucesso.".into(),
        })),
        Err(err) => Err((StatusCode::FORBIDDEN, err)),
    }
}

/// Agrega as rotas de administração.
pub fn admin_router(pool: Pool) -> Router {
    Router::new()
        .route("/add_admin", axum::routing::post(add_admin_handler))
        .route("/remove_admin", axum::routing::post(remove_admin_handler))
        .layer(Extension(pool))
}
