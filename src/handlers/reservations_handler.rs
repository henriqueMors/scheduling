use axum::{
    extract::{Extension, Json, Path}, // Adicionando `Path` aqui
    http::StatusCode,
};
use diesel::prelude::*;
use uuid::Uuid;
use crate::db::Pool;
use crate::models::reservation::{Reservation, NewReservation};
use crate::schema::reservations;
use crate::services::reservation_service;

#[axum::debug_handler]
pub async fn create_reservation(
    Extension(pool): Extension<Arc<Pool>>, // Passando Arc<Pool>
    Extension(user_id): Extension<Uuid>, // Obtém `user_id` autenticado via middleware
    Json(payload): Json<NewReservation>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    // Obtem a conexão do pool
    let mut conn = pool.get().map_err(|e| {
        error!("❌ Erro ao obter conexão com o banco de dados: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    // Criação da nova reserva
    let new_reservation = NewReservation {
        user_id,  // Usando `user_id` do middleware
        service: payload.service.clone(),
        appointment_time: payload.appointment_time,
        status: "pending".to_string(),
    };

    // Insere a reserva no banco de dados
    let reservation = diesel::insert_into(reservations::table)
        .values(&new_reservation)
        .get_result::<Reservation>(&mut conn)
        .map_err(|e| {
            error!("❌ Erro ao criar a reserva: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok(Json(reservation))  // Retorna a reserva criada
}

#[axum::debug_handler]
pub async fn get_reservation(
    Extension(pool): Extension<Arc<Pool>>, // Passando Arc<Pool>
    Path(reservation_id): Path<Uuid>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    // Obtem a conexão do pool
    let mut conn = pool.get().map_err(|e| {
        error!("❌ Erro ao obter conexão com o banco de dados: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    // Obtém uma reserva específica
    let reservation = reservation_service::get_reservation_by_id(&mut conn, reservation_id)
        .map_err(|e| {
            error!("❌ Erro ao buscar reserva: {:?}", e);
            (StatusCode::NOT_FOUND, "Reserva não encontrada".to_string())
        })
