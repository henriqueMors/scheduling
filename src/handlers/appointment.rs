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

/// 🔹 Creates a new appointment
pub async fn create_appointment(
    Extension(pool): Extension<Arc<Pool>>,  // Using Arc for thread safety
    Json(payload): Json<NewAppointment>,
) -> Result<Json<Appointment>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database connection error: {}", e)))?;

    let new_appointment = diesel::insert_into(appointments)
        .values(&payload)
        .get_result::<Appointment>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Insert error: {}", e)))?;

    Ok(Json(new_appointment))
}

/// 🔹 Lists all appointments for a client
pub async fn list_appointments_by_client(
    Extension(pool): Extension<Arc<Pool>>,
    Path(client_uuid): Path<Uuid>,  // Renamed to avoid confusion with column name
) -> Result<Json<Vec<Appointment>>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database connection error: {}", e)))?;

    let appointments_list = appointments
        .filter(client_id.eq(client_uuid))  // Clear distinction between column and variable
        .load::<Appointment>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Query error: {}", e)))?;

    if appointments_list.is_empty() {
        return Err((StatusCode::NOT_FOUND, "No appointments found for this client".into()));
    }

    Ok(Json(appointments_list))
}

/// 🔹 Updates an appointment status
pub async fn update_appointment(
    Extension(pool): Extension<Arc<Pool>>,
    Path(appointment_id): Path<Uuid>,
    Json(update): Json<UpdateAppointment>,
) -> Result<Json<Appointment>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database connection error: {}", e)))?;

    let updated_appointment = diesel::update(appointments)
        .filter(id.eq(appointment_id))
        .set(update)
        .get_result::<Appointment>(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => 
                (StatusCode::NOT_FOUND, "Appointment not found".into()),
            _ => 
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Update error: {}", e)),
        })?;

    Ok(Json(updated_appointment))
}

/// 🔹 Deletes an appointment
pub async fn delete_appointment(
    Extension(pool): Extension<Arc<Pool>>,
    Path(appointment_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database connection error: {}", e)))?;

    let rows_affected = diesel::delete(appointments)
        .filter(id.eq(appointment_id))
        .execute(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Delete error: {}", e)))?;

    if rows_affected == 0 {
        return Err((StatusCode::NOT_FOUND, "Appointment not found".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}