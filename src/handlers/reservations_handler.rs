use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use diesel::prelude::*;
use uuid::Uuid;
use crate::db::Pool;
use crate::models::reservation::{Reservation, NewReservation};
use crate::models::client::Client;
use crate::schema::{reservations, clients};

#[axum::debug_handler]
pub async fn create_reservation(
    Extension(pool): Extension<Pool>,
    Extension(user_id): Extension<Uuid>, // 🔹 Obtém `user_id` autenticado via middleware
    Json(payload): Json<NewReservation>,
) -> Result<Json<Reservation>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 🔹 Verifica se o usuário tem um `client_id` associado
    let client = clients::table
        .filter(clients::id.eq(user_id))
        .first::<Client>(&mut conn)
        .map_err(|_| (StatusCode::NOT_FOUND, "Client not found for this user".to_string()))?;

        let new_reservation = NewReservation {
            client_id: user_id,  // 🔹 Usa o `user_id` autenticado
            service: payload.service,
            appointment_time: payload.appointment_time,
            status: "pending".to_string(),  // 🔹 Status inicial
        };

    let reservation = diesel::insert_into(reservations::table)
        .values(&new_reservation)
        .get_result::<Reservation>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(reservation))
}
