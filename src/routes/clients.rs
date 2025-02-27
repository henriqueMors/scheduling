use axum::{
    extract::{Extension, Path, Json},
    routing::{get, post, put, delete},
    Router,
    http::StatusCode,
};
use uuid::Uuid;
use serde_json::json;
use diesel::prelude::*;
use crate::db::Pool;
use crate::models::client::{Client, NewClient, UpdateClient};
use crate::services::client_service;

/// Endpoint para criar um novo cliente.
pub async fn create_client(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<NewClient>,
) -> Result<Json<Client>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    client_service::create_client(&mut conn, payload)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Endpoint para obter os detalhes de um cliente específico.
pub async fn get_client(
    Extension(pool): Extension<Pool>,
    Path(client_id): Path<Uuid>,
) -> Result<Json<Client>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    client_service::get_client_by_id(&mut conn, client_id)
        .map(Json)
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))
}

/// Endpoint para listar todos os clientes.
pub async fn get_clients(
    Extension(pool): Extension<Pool>,
) -> Result<Json<Vec<Client>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    client_service::get_all_clients(&mut conn)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Endpoint para atualizar os dados de um cliente.
pub async fn update_client(
    Extension(pool): Extension<Pool>,
    Path(client_id): Path<Uuid>,
    Json(payload): Json<UpdateClient>, // Alterado para UpdateClient para atualizar o cliente
) -> Result<Json<Client>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    client_service::update_client(&mut conn, client_id, payload)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Endpoint para deletar um cliente.
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
        .route("/", get(get_clients).post(create_client))  // Rota de listagem de clientes e criação
        .route("/{client_id}", get(get_client).put(update_client).delete(delete_client)) // Rota de detalhes, atualização e remoção
        .layer(Extension(pool))
}
