use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    routing::{get, post, put, delete},
    Router,
};
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    db::Pool,
    models::service::{Service, NewService, UpdateService},
    schema::services::dsl::*,
};

/// 🔹 Cria um novo serviço (somente admin)
pub async fn create_service(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<NewService>,
) -> Result<Json<Service>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let new_service = diesel::insert_into(services)
        .values(&payload)
        .get_result::<Service>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(new_service))
}

/// 🔹 Lista todos os serviços
pub async fn list_services(
    Extension(pool): Extension<Pool>,
) -> Result<Json<Vec<Service>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let results = services
        .filter(ativo.eq(true)) // Apenas serviços ativos
        .load::<Service>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(results))
}

/// 🔹 Busca serviço por ID
pub async fn get_service_by_id(
    Extension(pool): Extension<Pool>,
    Path(service_id): Path<Uuid>,
) -> Result<Json<Service>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let service = services
        .filter(id.eq(service_id))
        .first::<Service>(&mut conn)
        .map_err(|_| (StatusCode::NOT_FOUND, "Service not found".to_string()))?;

    Ok(Json(service))
}

/// 🔹 Atualiza serviço
pub async fn update_service(
    Extension(pool): Extension<Pool>,
    Path(service_id): Path<Uuid>,
    Json(update): Json<UpdateService>,
) -> Result<Json<Service>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let updated_service = diesel::update(services.filter(id.eq(service_id)))
        .set(update)
        .get_result::<Service>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(updated_service))
}

/// 🔹 Deleta serviço
pub async fn delete_service(
    Extension(pool): Extension<Pool>,
    Path(service_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    diesel::delete(services.filter(id.eq(service_id)))
        .execute(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
