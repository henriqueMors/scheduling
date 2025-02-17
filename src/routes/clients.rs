use axum::{
    extract::{Path, Extension},
    Json, Router,
    routing::{get, post, put, delete},
    http::StatusCode,
};
use uuid::Uuid;
use serde_json::json;
use crate::models::client::{Client, NewClient, UpdateClient};
use crate::services::client_service;
use crate::db::Pool;

/// Handler para criar um novo cliente.
/// Endpoint: POST /clients
pub async fn create_client(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<NewClient>,
) -> Result<Json<Client>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match client_service::create_client(&mut conn, payload) {
        Ok(client) => Ok(Json(client)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Handler para buscar um cliente pelo ID.
/// Endpoint: GET /clients/:id
pub async fn get_client(
    Extension(pool): Extension<Pool>,
    Path(client_id): Path<Uuid>,
) -> Result<Json<Client>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match client_service::get_client_by_id(&mut conn, client_id) {
        Ok(client) => Ok(Json(client)),
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

/// Handler para retornar todos os clientes.
/// Endpoint: GET /clients
pub async fn get_clients(
    Extension(pool): Extension<Pool>,
) -> Result<Json<Vec<Client>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match client_service::get_all_clients(&mut conn) {
        Ok(clients) => Ok(Json(clients)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Handler para atualizar um cliente.
/// Endpoint: PUT /clients/:id
pub async fn update_client(
    Extension(pool): Extension<Pool>,
    Path(client_id): Path<Uuid>,
    Json(payload): Json<UpdateClient>,
) -> Result<Json<Client>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match client_service::update_client(&mut conn, client_id, payload) {
        Ok(client) => Ok(Json(client)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Handler para deletar um cliente.
/// Endpoint: DELETE /clients/:id
pub async fn delete_client(
    Extension(pool): Extension<Pool>,
    Path(client_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match client_service::delete_client(&mut conn, client_id) {
        Ok(deleted) if deleted > 0 => Ok(Json(json!({"message": "Client deleted"}))),
        Ok(_) => Err((StatusCode::NOT_FOUND, "Client not found".into())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Agrega as rotas de clientes em um Router do Axum.
pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/", get(get_clients).post(create_client))
        .route("/:id", get(get_client).put(update_client).delete(delete_client))
        .layer(Extension(pool))
}
