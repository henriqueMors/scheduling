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
    Extension(role): Extension<String>, 
    Json(payload): Json<NewReservation>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(map_db_error)?;

    // 🔒 Restrição para `client`: só pode criar reservas para ele mesmo
    if role == "client" && payload.user_id != user_id {
        return Err((StatusCode::FORBIDDEN, "Clients can only create their own reservations.".to_string()));
    }

    reservation_service::create_reservation(&mut conn, payload)
        .map(Json)
        .map_err(map_internal_error)
}

/// 🔹 Busca uma reserva específica por ID.
pub async fn get_reservation(
    Extension(pool): Extension<Pool>,
    Extension(user_id): Extension<Uuid>, 
    Extension(role): Extension<String>, 
    Path(reservation_id): Path<Uuid>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(map_db_error)?;

    let reservation = reservation_service::get_reservation_by_id(&mut conn, reservation_id)
        .map_err(map_not_found_error)?;

    // 🔒 Clients só podem acessar suas próprias reservas
    if role == "client" && reservation.user_id != user_id {
        return Err((StatusCode::FORBIDDEN, "You are not allowed to access this reservation.".to_string()));
    }

    Ok(Json(reservation))
}

/// 🔹 Lista todas as reservas.
pub async fn get_reservations(
    Extension(pool): Extension<Pool>,
    Extension(user_id): Extension<Uuid>, 
    Extension(role): Extension<String>,
) -> Result<Json<Vec<Reservation>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(map_db_error)?;

    let reservations = if role == "client" {
        // 🔒 Clients só podem ver suas próprias reservas
        reservation_service::list_reservations_by_user(&mut conn, user_id)
    } else {
        // 🔒 Admins e Admin Masters podem ver tudo
        reservation_service::list_reservations(&mut conn)
    };

    reservations
        .map(Json)
        .map_err(map_internal_error)
}

/// 🔹 Atualiza uma reserva existente.
pub async fn update_reservation(
    Extension(pool): Extension<Pool>,
    Extension(user_id): Extension<Uuid>, 
    Extension(role): Extension<String>, 
    Path(reservation_id): Path<Uuid>,
    Json(payload): Json<UpdateReservation>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(map_db_error)?;

    let existing_reservation = reservation_service::get_reservation_by_id(&mut conn, reservation_id)
        .map_err(map_not_found_error)?;

    // 🔒 Clients só podem atualizar suas próprias reservas
    if role == "client" && existing_reservation.user_id != user_id {
        return Err((StatusCode::FORBIDDEN, "You are not allowed to update this reservation.".to_string()));
    }

    reservation_service::update_reservation(&mut conn, reservation_id, payload)
        .map(Json)
        .map_err(map_internal_error)
}

/// 🔹 Remove uma reserva por ID.
pub async fn delete_reservation(
    Extension(pool): Extension<Pool>,
    Extension(user_id): Extension<Uuid>, 
    Extension(role): Extension<String>, 
    Path(reservation_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(map_db_error)?;

    let existing_reservation = reservation_service::get_reservation_by_id(&mut conn, reservation_id)
        .map_err(map_not_found_error)?;

    // 🔒 Clients só podem excluir suas próprias reservas
    if role == "client" && existing_reservation.user_id != user_id {
        return Err((StatusCode::FORBIDDEN, "You are not allowed to delete this reservation.".to_string()));
    }

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
            get(get_reservations)
                .post(create_reservation)
                .layer(from_fn(|req, next| require_role("client".to_string(), req, next))),
        )
        .route(
            "/:reservation_id",
            get(get_reservation)
                .put(update_reservation)
                .delete(delete_reservation)
                .layer(from_fn(|req, next| require_role("client".to_string(), req, next))),
        )
        .layer(Extension(pool))
}
