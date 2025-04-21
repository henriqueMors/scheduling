use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    routing::{get, post, put, delete},
    Router,
};
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{NaiveDate, NaiveTime};

use crate::{
    db::Pool,
    models::availability::{Availability, NewAvailability, UpdateAvailability},
    schema::availabilities::dsl::*,
};

// 🔹 Cria um novo horário disponível
pub async fn create_availability(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<NewAvailability>,
) -> Result<Json<Availability>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let new_availability = diesel::insert_into(availabilities)
        .values(&payload)
        .get_result::<Availability>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(new_availability))
}

// 🔹 Lista todos os horários disponíveis de um profissional
pub async fn list_availabilities_by_professional(
    Extension(pool): Extension<Pool>,
    Path(professional_uuid): Path<Uuid>,
) -> Result<Json<Vec<Availability>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let availability_list = availabilities
        .filter(professional_id.eq(professional_uuid))
        .load::<Availability>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(availability_list))
}

// 🔹 Atualiza um horário disponível
pub async fn update_availability(
    Extension(pool): Extension<Pool>,
    Path(availability_id): Path<Uuid>,
    Json(update): Json<UpdateAvailability>,
) -> Result<Json<Availability>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let updated_availability = diesel::update(availabilities.filter(id.eq(availability_id)))
        .set(update)
        .get_result::<Availability>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(updated_availability))
}

// 🔹 Deleta um horário disponível
pub async fn delete_availability(
    Extension(pool): Extension<Pool>,
    Path(availability_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    diesel::delete(availabilities.filter(id.eq(availability_id)))
        .execute(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

// 🔹 Configuração das rotas
pub fn availability_routes() -> Router {
    Router::new()
        .route("/availabilities", post(create_availability))
        .route("/availabilities/:professional_id", get(list_availabilities_by_professional))
        .route("/availabilities/:id", put(update_availability))
        .route("/availabilities/:id", delete(delete_availability))
}