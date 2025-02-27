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
use crate::models::reservation::{Reservation, NewReservation, UpdateReservation};
use crate::services::reservation_service;

/// Endpoint para criar uma reserva.
pub async fn create_reservation(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<NewReservation>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    reservation_service::create_reservation(&mut conn, payload)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Endpoint para buscar uma reserva específica.
pub async fn get_reservation(
    Extension(pool): Extension<Pool>,
    Path(reservation_id): Path<Uuid>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    reservation_service::get_reservation_by_id(&mut conn, reservation_id)
        .map(Json)
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))
}

/// Endpoint para listar todas as reservas.
pub async fn get_reservations(
    Extension(pool): Extension<Pool>,
) -> Result<Json<Vec<Reservation>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    reservation_service::list_reservations(&mut conn)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Endpoint para atualizar uma reserva.
pub async fn update_reservation(
    Extension(pool): Extension<Pool>,
    Path(reservation_id): Path<Uuid>,
    Json(payload): Json<UpdateReservation>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    reservation_service::update_reservation(&mut conn, reservation_id, payload)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Endpoint para deletar uma reserva.
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
        .route("/", get(get_reservations).post(create_reservation))
        .route("/:id", get(get_reservation).put(update_reservation).delete(delete_reservation))
        .layer(Extension(pool))
}
