use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
};
use diesel::prelude::*;
use uuid::Uuid;
use crate::db::Pool;
use crate::models::reservation::{Reservation, NewReservation};
use crate::schema::reservations;
use crate::services::reservation_service;
use tracing::error;
use std::sync::Arc;  // Certifique-se de importar `Arc`

#[axum::debug_handler]
pub async fn create_reservation(
    Extension(pool): Extension<Arc<Pool>>,  // Recebendo Arc<Pool>
    Extension(user_id): Extension<Uuid>,   // Obtém `user_id` autenticado via middleware
    Json(payload): Json<NewReservation>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    // Obtem a conexão do pool (desembrulhando o Arc para acessar o Pool)
    let mut conn = pool.as_ref().get().map_err(|e| {
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
    Extension(pool): Extension<Arc<Pool>>,  // Recebendo Arc<Pool>
    Path(reservation_id): Path<Uuid>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    // Obtem a conexão do pool (desembrulhando o Arc para acessar o Pool)
    let mut conn = pool.as_ref().get().map_err(|e| {
        error!("❌ Erro ao obter conexão com o banco de dados: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    // Obtém uma reserva específica
    let reservation = reservation_service::get_reservation_by_id(&mut conn, reservation_id)
        .map_err(|e| {
            error!("❌ Erro ao buscar reserva: {:?}", e);
            (StatusCode::NOT_FOUND, "Reserva não encontrada".to_string())
        })?;

    Ok(Json(reservation)) // Retorna a reserva encontrada
}

#[axum::debug_handler]
pub async fn get_all_reservations(
    Extension(pool): Extension<Arc<Pool>>,  // Recebendo Arc<Pool>
) -> Result<Json<Vec<Reservation>>, (StatusCode, String)> {
    // Obtem a conexão do pool (desembrulhando o Arc para acessar o Pool)
    let mut conn = pool.as_ref().get().map_err(|e| {
        error!("❌ Erro ao obter conexão com o banco de dados: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    // Obtém todas as reservas
    let all_reservations = reservations::table
        .load::<Reservation>(&mut conn)
        .map_err(|e| {
            error!("❌ Erro ao buscar reservas: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok(Json(all_reservations)) // Retorna todas as reservas encontradas
}
