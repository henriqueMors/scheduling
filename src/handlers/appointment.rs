use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    routing::{post, get, put, delete},
    Router,
};
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::{
    db::Pool,
    models::appointment::{Appointment, NewAppointment, UpdateAppointment},
    schema::appointments::dsl::*,
};

/// 🔹 Cria um novo agendamento
pub async fn create_appointment(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<NewAppointment>,
) -> Result<Json<Appointment>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let new_appointment = diesel::insert_into(appointments)
        .values(&payload)
        .get_result::<Appointment>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(new_appointment))
}


/// 🔹 Lista todos os agendamentos de um cliente
pub async fn list_appointments_by_client(
    Extension(pool): Extension<Pool>,
    Path(client_id): Path<Uuid>,  // Extraindo o client_id da URL como `Uuid`
) -> Result<Json<Vec<Appointment>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Agora com a definição correta do tipo `client_id` no schema
    let appointments_list = appointments
        .filter(client_id.eq(client_id))  // Usando `client_id` extraído corretamente da URL
        .load::<Appointment>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(appointments_list))
}

/// 🔹 Atualiza o status de um agendamento
pub async fn update_appointment(
    Extension(pool): Extension<Pool>,
    Path(appointment_id): Path<Uuid>,
    Json(update): Json<UpdateAppointment>,
) -> Result<Json<Appointment>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let updated_appointment = diesel::update(appointments.filter(id.eq(appointment_id)))
        .set(update)
        .get_result::<Appointment>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(updated_appointment))
}

/// 🔹 Deleta um agendamento
pub async fn delete_appointment(
    Extension(pool): Extension<Pool>,
    Path(appointment_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    diesel::delete(appointments.filter(id.eq(appointment_id)))
        .execute(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
