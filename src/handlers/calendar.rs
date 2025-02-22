use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    Json, Router,
};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime, Duration as ChronoDuration};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::db::Pool;
use crate::models::reservation::Reservation; // Supondo que exista
use crate::schema::reservations::dsl::*;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CalendarQuery {
    pub date: String, // Espera-se uma data no formato "YYYY-MM-DD"
}

#[derive(Serialize)]
pub struct TimeSlot {
    pub time: String,    // Ex.: "08:00", "08:30", etc.
    pub status: String,  // "disponível" ou "indisponível"
    // Para administradores, podemos incluir detalhes adicionais:
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_details: Option<ReservationDetails>,
}

#[derive(Serialize)]
pub struct ReservationDetails {
    pub reservation_id: Uuid,
    pub client_id: Uuid,
    // Adicione outros campos que o administrador precise visualizar
}

#[derive(Serialize)]
pub struct CalendarResponse {
    pub date: String,
    pub slots: Vec<TimeSlot>,
}

/// Endpoint GET /calendar?date=YYYY-MM-DD
/// Retorna o calendário para a data especificada.
/// A resposta varia conforme o papel do usuário (admin ou client).
pub async fn get_calendar(
    Extension(pool): Extension<Pool>,
    Query(query): Query<CalendarQuery>,
    // Em uma implementação real, você extrairia o token JWT do header e decodificaria para obter o role do usuário.
    // Aqui, vamos simular com uma variável extra.
) -> Result<Json<CalendarResponse>, (StatusCode, String)> {
    // Parse da data
    let date = NaiveDate::parse_from_str(&query.date, "%Y-%m-%d")
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid date format".into()))?;
    
    // Definindo horário de funcionamento (ex.: 08:00 às 18:00, com intervalos de 30 minutos)
    let start_time = NaiveTime::from_hms(8, 0, 0);
    let end_time = NaiveTime::from_hms(18, 0, 0);
    let slot_duration = ChronoDuration::minutes(30);
    
    let mut slots = Vec::new();
    let mut current_time = NaiveDateTime::new(date, start_time);
    let end_datetime = NaiveDateTime::new(date, end_time);
    
    // Simule a extração do role do usuário a partir do token JWT (por exemplo, "admin" ou "client").
    // Aqui, para exemplo, vamos assumir que é "client". Altere conforme necessário.
    let user_role = "client"; // ou "admin"
    
    // Obtenha a lista de reservas para o dia
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let day_reservations: Vec<Reservation> = reservations
        .filter(appointment_time.ge(current_time))
        .filter(appointment_time.lt(end_datetime))
        .load(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Itera sobre os intervalos de tempo do dia
    while current_time < end_datetime {
        // Verifica se existe uma reserva para o horário atual
        let reservation_opt = day_reservations.iter().find(|r| {
            // Supondo que o campo appointment_time seja um NaiveDateTime
            r.appointment_time == current_time
        });
        
        if let Some(reservation) = reservation_opt {
            // Se existir reserva, o slot é "indisponível"
            // Para administradores, incluir detalhes da reserva
            let details = if user_role == "admin" {
                Some(ReservationDetails {
                    reservation_id: reservation.id,
                    client_id: reservation.client_id,
                })
            } else {
                None
            };
            slots.push(TimeSlot {
                time: current_time.time().format("%H:%M").to_string(),
                status: "indisponível".into(),
                reservation_details: details,
            });
        } else {
            // Slot disponível
            slots.push(TimeSlot {
                time: current_time.time().format("%H:%M").to_string(),
                status: "disponível".into(),
                reservation_details: None,
            });
        }
        current_time += slot_duration;
    }
    
    Ok(Json(CalendarResponse {
        date: query.date,
        slots,
    }))
}

/// Agrega as rotas do calendário.
pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/", axum::routing::get(get_calendar))
        .layer(Extension(pool))
}
