use axum::{routing::{get, post, put, delete}, Router, Json, extract::{Path, State}};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use serde::{Deserialize, Serialize};  // ✅ Corrigido
use uuid::Uuid;
use chrono::NaiveDateTime;  // ✅ Corrigido
use crate::{db::DbPool, models::reservation::{Reservation, NewReservation}, schema::reservations::dsl::*};

pub fn router(pool: DbPool) -> Router {
    Router::new()
        .route("/", get(list_reservations).post(create_reservation))
        .route("/:id", get(get_reservation).put(update_reservation).delete(delete_reservation))
        .with_state(pool)
}

// 🔹 Listar todas as reservas (GET /reservations)
async fn list_reservations(State(pool): State<DbPool>) -> Json<Vec<Reservation>> {
    let mut conn = pool.get().expect("Falha ao obter conexão do banco");
    let results = reservations.load::<Reservation>(&mut conn).expect("Erro ao buscar reservas");
    Json(results)
}

// 🔹 Criar nova reserva (POST /reservations)
#[derive(Deserialize)]
struct CreateReservation {
    client_id: Uuid,
    service: String,
    appointment_time: NaiveDateTime,
    status: String,
}

async fn create_reservation(State(pool): State<DbPool>, Json(payload): Json<CreateReservation>) -> Json<Reservation> {
    let mut conn = pool.get().expect("Falha ao obter conexão do banco");

    let new_reservation = NewReservation {
        id: Uuid::new_v4(),
        client_id: payload.client_id,
        service: payload.service,
        appointment_time: payload.appointment_time,
        status: payload.status,
    };

    diesel::insert_into(reservations)
        .values(&new_reservation)
        .execute(&mut conn)
        .expect("Erro ao inserir reserva");

    Json(Reservation {
        id: new_reservation.id,
        client_id: new_reservation.client_id,
        service: new_reservation.service,
        appointment_time: new_reservation.appointment_time,
        status: new_reservation.status,
    })
}

// 🔹 Buscar reserva pelo ID (GET /reservations/:id)
async fn get_reservation(State(pool): State<DbPool>, Path(reservation_id): Path<Uuid>) -> Json<Reservation> {
    let mut conn = pool.get().expect("Falha ao obter conexão do banco");
    let reservation = reservations
        .filter(id.eq(reservation_id))
        .first::<Reservation>(&mut conn)
        .expect("Reserva não encontrada");

    Json(reservation)
}

// 🔹 Atualizar reserva (PUT /reservations/:id)
#[derive(Deserialize)]
struct UpdateReservation {
    service: Option<String>,
    appointment_time: Option<NaiveDateTime>,
    status: Option<String>,
}

async fn update_reservation(State(pool): State<DbPool>, Path(reservation_id): Path<Uuid>, Json(payload): Json<UpdateReservation>) -> Json<Reservation> {
    let mut conn = pool.get().expect("Falha ao obter conexão do banco");

    diesel::update(reservations.filter(id.eq(reservation_id)))
        .set((
            service.eq(payload.service.unwrap_or_else(|| "".to_string())),
            appointment_time.eq(payload.appointment_time.unwrap_or_else(|| chrono::Utc::now().naive_utc())),
            status.eq(payload.status.unwrap_or_else(|| "pending".to_string())),
        ))
        .execute(&mut conn)
        .expect("Erro ao atualizar reserva");

    let updated_reservation = reservations
        .filter(id.eq(reservation_id))
        .first::<Reservation>(&mut conn)
        .expect("Reserva não encontrada");

    Json(updated_reservation)
}

// 🔹 Deletar reserva (DELETE /reservations/:id)
async fn delete_reservation(State(pool): State<DbPool>, Path(reservation_id): Path<Uuid>) -> Json<String> {
    let mut conn = pool.get().expect("Falha ao obter conexão do banco");

    diesel::delete(reservations.filter(id.eq(reservation_id)))
        .execute(&mut conn)
        .expect("Erro ao deletar reserva");

    Json(format!("Reserva {} removida!", reservation_id))
}
