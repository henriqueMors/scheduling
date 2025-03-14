use axum::{
    extract::{Extension, Path, Json},
    routing::{get, post, put, delete},
    Router,
    http::StatusCode,
    middleware::from_fn,
};
use uuid::Uuid;
use serde_json::json;
use diesel::prelude::*;
use crate::db::Pool;
use crate::models::reservation::{Reservation, NewReservation, UpdateReservation};
use crate::services::reservation_service;
use crate::middleware::auth_middleware::{require_role};


/// 🔹 Cria uma reserva.
pub async fn create_reservation(
    Extension(pool): Extension<Pool>,
    Extension(user_id): Extension<Uuid>, 
    Json(payload): Json<NewReservation>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(map_db_error)?;

    // Criação da reserva
    reservation_service::create_reservation(&mut conn, payload)
        .map(Json)
        .map_err(map_internal_error)
}

/// 🔹 Busca uma reserva específica por ID.
pub async fn get_reservation(
    Extension(pool): Extension<Pool>,
    Path(reservation_id): Path<Uuid>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(map_db_error)?;

    let reservation = reservation_service::get_reservation_by_id(&mut conn, reservation_id)
        .map_err(map_not_found_error)?;

    Ok(Json(reservation))
}

/// 🔹 Lista todas as reservas.
pub async fn get_all_reservations(
    Extension(pool): Extension<Pool>,
) -> Result<Json<Vec<Reservation>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(map_db_error)?;

    let all_reservations = reservation_service::list_reservations(&mut conn)
        .map_err(map_internal_error)?;

    Ok(Json(all_reservations))
}

/// 🔹 Atualiza uma reserva existente.
pub async fn update_reservation(
    Extension(pool): Extension<Pool>,
    Path(reservation_id): Path<Uuid>,
    Json(payload): Json<UpdateReservation>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(map_db_error)?;

    reservation_service::update_reservation(&mut conn, reservation_id, payload)
        .map(Json)
        .map_err(map_internal_error)
}

/// 🔹 Remove uma reserva por ID.
pub async fn delete_reservation(
    Extension(pool): Extension<Pool>,
    Path(reservation_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(map_db_error)?;

    match reservation_service::delete_reservation(&mut conn, reservation_id) {
        Ok(deleted) if deleted > 0 => Ok(Json(json!({"message": "Reservation deleted"}))),
        Ok(_) => Err((StatusCode::NOT_FOUND, "Reservation not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// 🔹 Mapeia erros de banco de dados (Diesel)
fn map_db_error(e: diesel::r2d2::PoolError) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get DB connection: {}", e))
}

/// 🔹 Mapeia erros internos
fn map_internal_error(e: diesel::result::Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
}

/// 🔹 Mapeia erro de "não encontrado"
fn map_not_found_error(e: diesel::result::Error) -> (StatusCode, String) {
    match e {
        diesel::result::Error::NotFound => (StatusCode::NOT_FOUND, "Reservation not found".to_string()),
        _ => map_internal_error(e),
    }
}

/// 🔹 Agrega as rotas de reservas.
pub fn router(pool: Pool) -> Router {
    Router::new()
        .route(
            "/",
            get(get_all_reservations) // ✅ Rota para listar todas as reservas
                .post(create_reservation),
        )
        .route(
            "/:reservation_id",
            get(get_reservation)
                .put(update_reservation)
                .delete(delete_reservation),
        )
        .layer(Extension(pool))
}
