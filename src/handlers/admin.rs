use axum::{extract::{Extension, Json}, http::StatusCode, Router};
use diesel::prelude::*;
use crate::db::Pool;
use crate::models::user::{User, NewUser};
use crate::services::admin_service::{add_admin, remove_admin};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AddAdminRequest {
    pub name: String,
    pub phone: String,
    pub password: String, // Aqui você pode hash a senha antes de salvar no banco
}

#[derive(Deserialize)]
pub struct RemoveAdminRequest {
    pub admin_id: Uuid,
}

#[derive(Serialize)]
pub struct AdminResponse {
    pub message: String,
}

/// Endpoint para adicionar um novo administrador
pub async fn add_admin_handler(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<AddAdminRequest>,
) -> Result<Json<AdminResponse>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Aqui, você precisa usar o ID do administrador master (que será obtido do token JWT, por exemplo)
    let master_id = Uuid::new_v4(); // Isso deve ser substituído pela lógica que extrai o ID do token

    let password_hash = "hashed_password"; // Aqui você deve hash a senha utilizando argon2 ou outro algoritmo

    match add_admin(&mut conn, master_id, payload.name.clone(), payload.phone.clone(), password_hash.to_string()) {
        Ok(user) => Ok(Json(AdminResponse {
            message: format!("Administrador {} adicionado com sucesso.", user.name),
        })),
        Err(err) => Err((StatusCode::FORBIDDEN, err)),
    }
}

/// Endpoint para remover um administrador
pub async fn remove_admin_handler(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<RemoveAdminRequest>,
) -> Result<Json<AdminResponse>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Aqui, você precisa usar o ID do administrador master (que será obtido do token JWT, por exemplo)
    let master_id = Uuid::new_v4(); // Isso deve ser substituído pela lógica que extrai o ID do token

    match remove_admin(&mut conn, master_id, payload.admin_id) {
        Ok(_) => Ok(Json(AdminResponse {
            message: "Administrador removido com sucesso.".to_string(),
        })),
        Err(err) => Err((StatusCode::FORBIDDEN, err)),
    }
}

/// Agrega as rotas de administração
pub fn admin_router(pool: Pool) -> Router {
    Router::new()
        .route("/add_admin", axum::routing::post(add_admin_handler))
        .route("/remove_admin", axum::routing::post(remove_admin_handler))
        .layer(Extension(pool))
}
