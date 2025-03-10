use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use diesel::prelude::*;
use uuid::Uuid;
use crate::db::Pool;
use crate::models::reservation::{Reservation, NewReservation};
use crate::schema::reservations;

#[axum::debug_handler]
pub async fn create_reservation(
    Extension(pool): Extension<Pool>,
    Extension(user_id): Extension<Uuid>, // ✅ Obtém `user_id` autenticado via middleware
    Json(payload): Json<NewReservation>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // ✅ Criação correta da `NewReservation`
    let new_reservation = NewReservation {
        user_id, // ✅ Agora usa `user_id` diretamente
        service: payload.service.clone(), // ✅ Clona `String`
        appointment_time: payload.appointment_time.clone(), // ✅ Clona `String`
        status: "pending".to_string(),  // ✅ Status inicial como String
    };

    // ✅ Insere a reserva no banco
    let reservation = diesel::insert_into(reservations::table)
        .values(&new_reservation)
        .get_result::<Reservation>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(reservation))
}
