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

    let new_availability = diesel::insert_into(availabilities)  // Aqui está a tabela `availabilities` 
        .values(&payload)
        .get_result::<Availability>(&mut conn) // A consulta é feita em `availabilities` e não na tabela diretamente
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(new_availability))
}

// 🔹 Lista todos os horários disponíveis de um profissional
pub async fn list_availabilities_by_professional(
    Extension(pool): Extension<Pool>,
    Path(professional_uuid): Path<Uuid>,  // Renomeando para evitar conflito com a coluna
) -> Result<Json<Vec<Availability>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Filtrando pela coluna `professional_id` da tabela `availabilities`
    let availabilities = availabilities
        .filter(professional_id.eq(professional_uuid))  // Aqui, estamos filtrando pelas colunas corretamente
        .load::<Availability>(&mut conn)  // O Diesel sabe que você quer os dados da tabela `availabilities`
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(availabilities))
}

// 🔹 Atualiza um horário disponível
pub async fn update_availability(
    Extension(pool): Extension<Pool>,
    Path(other_id): Path<Uuid>,  // Renomeando para evitar conflito com a coluna `id`
    Json(update): Json<UpdateAvailability>,
) -> Result<Json<Availability>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let updated_availability = diesel::update(availabilities.filter(id.eq(other_id)))  // Corrigindo o uso de `id`
        .set(update)
        .get_result::<Availability>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(updated_availability))
}

// 🔹 Deleta um horário disponível
pub async fn delete_availability(
    Extension(pool): Extension<Pool>,
    Path(other_id): Path<Uuid>,  // Renomeando para evitar conflito com a coluna `id`
) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    diesel::delete(availabilities.filter(id.eq(other_id)))  // Corrigindo o uso de `id`
        .execute(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
