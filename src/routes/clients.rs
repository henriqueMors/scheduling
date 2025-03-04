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

/// 🔹 Cria um novo cliente
pub async fn create_client(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<NewClient>,
) -> Result<Json<Client>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database connection error: {}", e)))?;

    client_service::create_client(&mut conn, payload)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", e)))
}

/// 🔹 Obtém os detalhes de um cliente específico
pub async fn get_client(
    Extension(pool): Extension<Pool>,
    Path(client_id): Path<Uuid>,
) -> Result<Json<Client>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database connection error: {}", e)))?;

    client_service::get_client_by_id(&mut conn, client_id)
        .map(Json)
        .map_err(|_| (StatusCode::NOT_FOUND, "Client not found".to_string()))
}

/// 🔹 Lista todos os clientes
pub async fn get_clients(
    Extension(pool): Extension<Pool>,
) -> Result<Json<Vec<Client>>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database connection error: {}", e)))?;

    client_service::get_all_clients(&mut conn)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch clients: {}", e)))
}

/// 🔹 Atualiza os dados de um cliente
pub async fn update_client(
    Extension(pool): Extension<Pool>,
    Path(client_id): Path<Uuid>,
    Json(payload): Json<UpdateClient>,
) -> Result<Json<Client>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database connection error: {}", e)))?;

    client_service::update_client(&mut conn, client_id, payload)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to update client: {}", e)))
}

/// 🔹 Deleta um cliente
pub async fn delete_client(
    Extension(pool): Extension<Pool>,
    Path(client_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database connection error: {}", e)))?;

    match client_service::delete_client(&mut conn, client_id) {
        Ok(deleted) if deleted > 0 => Ok(Json(json!({"message": "Client deleted"}))),
        Ok(_) => Err((StatusCode::NOT_FOUND, "Client not found".into())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to delete client: {}", e))),
    }
}

/// 🔹 Define as rotas de `/clients`
pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/", get(get_clients).post(create_client))  
        .route(
            "/{client_id}",  // ✅ Corrigido! Agora usa `{client_id}`
            get(get_client).put(update_client).delete(delete_client),
        ) 
        .layer(Extension(pool))
}
