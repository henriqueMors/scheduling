use axum::{
    extract::{Path, Extension},
    Json, Router,
    routing::{get, post, put, delete},
    http::StatusCode,
};

use uuid::Uuid;
use serde_json::json;
use crate::db::Pool;
use crate::models::reservation::{Reservation, NewReservation, UpdateReservation};
use crate::services::reservation_service;

/// Handler para listar todas as reservas.
/// Endpoint: GET /reservations
pub async fn list_reservations(
    Extension(pool): Extension<Pool>,
) -> Result<Json<Vec<Reservation>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match reservation_service::list_reservations(&mut conn) {
        Ok(list) => Ok(Json(list)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Handler para criar uma nova reserva.
/// Endpoint: POST /reservations
pub async fn create_reservation(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<NewReservation>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match reservation_service::create_reservation(&mut conn, payload) {
        Ok(reservation) => Ok(Json(reservation)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Handler para buscar uma reserva pelo ID.
/// Endpoint: GET /reservations/:id
pub async fn get_reservation(
    Extension(pool): Extension<Pool>,
    Path(reservation_id): Path<Uuid>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match reservation_service::get_reservation_by_id(&mut conn, reservation_id) {
        Ok(reservation) => Ok(Json(reservation)),
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

/// Handler para atualizar uma reserva.
/// Endpoint: PUT /reservations/:id
pub async fn update_reservation(
    Extension(pool): Extension<Pool>,
    Path(reservation_id): Path<Uuid>,
    Json(payload): Json<UpdateReservation>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match reservation_service::update_reservation(&mut conn, reservation_id, payload) {
        Ok(reservation) => Ok(Json(reservation)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Handler para deletar uma reserva.
/// Endpoint: DELETE /reservations/:id
pub async fn delete_reservation(
    Extension(pool): Extension<Pool>,
    Path(reservation_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match reservation_service::delete_reservation(&mut conn, reservation_id) {
        Ok(deleted) if deleted > 0 => Ok(Json(json!({"message": "Reservation deleted"}))),
        Ok(_) => Err((StatusCode::NOT_FOUND, "Reservation not found".into())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Agrega as rotas de reservas em um Router do Axum.
pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/", get(list_reservations).post(create_reservation))
        .route("/:id", get(get_reservation).put(update_reservation).delete(delete_reservation))
        .layer(Extension(pool))
}
