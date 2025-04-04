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
use crate::middleware::auth_middleware::{AuthMiddleware, RequireRole};  // Importação corrigida
use crate::models::reservation::{Reservation, NewReservation, UpdateReservation};
use crate::services::reservation_service;

/// 🔹 Cria uma reserva.
pub async fn create_reservation(
    Extension(pool): Extension<Pool>,
    Extension(user_id): Extension<Uuid>,  // ✅ Obtém `user_id` autenticado via middleware
    Json(payload): Json<NewReservation>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(map_db_error)?;

    // Criação da reserva
    let new_reservation = NewReservation {
        user_id,  // ✅ AGORA USANDO `user_id`
        service: payload.service.clone(),
        appointment_time: payload.appointment_time,
        status: "pending".to_string(),
    };

    // ✅ Chama a função de serviço para criar a reserva no banco
    reservation_service::create_reservation(&mut conn, new_reservation)
        .map(Json)
        .map_err(map_internal_error)
}

/// 🔹 Busca uma reserva específica por ID.
pub async fn get_reservation(
    Extension(pool): Extension<Pool>,
    Extension(user_id): Extension<Uuid>,  // ✅ Obtém `user_id` autenticado via middleware
    Extension(role): Extension<String>,   // ✅ Obtém o papel do usuário (role)
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
    Extension(user_id): Extension<Uuid>,  // ✅ Obtém `user_id` autenticado via middleware
    Extension(role): Extension<String>,   // ✅ Obtém o papel do usuário (role)
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
    Extension(user_id): Extension<Uuid>,  // ✅ Obtém `user_id` autenticado via middleware
    Extension(role): Extension<String>,   // ✅ Obtém o papel do usuário (role)
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
    Extension(user_id): Extension<Uuid>,  // ✅ Obtém `user_id` autenticado via middleware
    Extension(role): Extension<String>,   // ✅ Obtém o papel do usuário (role)
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
    // Cria um ServiceBuilder com todas as camadas de middleware necessárias
    let middleware_stack = tower::ServiceBuilder::new()
        .layer(AuthMiddleware) // Primeiro verifica a autenticação
        .layer(RequireRole::new("client".to_string())); // Depois verifica a role

    Router::new()
        .route(
            "/",
            get(get_reservations)    // Rota GET para listar as reservas
                .post(create_reservation) // Rota POST para criar uma nova reserva
        )
        .route(
            "/:reservation_id",
            get(get_reservation)     // Rota GET para buscar uma reserva por ID
                .put(update_reservation)  // Rota PUT para atualizar uma reserva existente
                .delete(delete_reservation) // Rota DELETE para remover uma reserva
        )
        .layer(middleware_stack)
        .layer(Extension(pool)) // Compartilha o pool de conexões com o banco de dados
}