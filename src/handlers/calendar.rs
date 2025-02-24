use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    Json, Router,
};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime, Duration as ChronoDuration};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::db::Pool;
use crate::models::reservation::Reservation; // Certifique-se de que o modelo Reservation está implementado
use crate::schema::reservations::dsl::*; // Presume que sua tabela de reservas possui uma coluna "appointment_time"
use uuid::Uuid;

/// Estrutura para receber a data via query parameter
#[derive(Deserialize)]
pub struct CalendarQuery {
    /// Data no formato "YYYY-MM-DD"
    pub date: String,
}

/// Estrutura que representa os detalhes de uma reserva para exibição (para administradores)
#[derive(Serialize)]
pub struct ReservationDetails {
    pub reservation_id: String,
    pub client_id: String,
    // Adicione outros campos conforme necessário
}

/// Estrutura para representar um slot de tempo no calendário
#[derive(Serialize)]
pub struct TimeSlot {
    /// Horário do slot, ex: "08:00"
    pub time: String,
    /// Status: "disponível" ou "indisponível"
    pub status: String,
    /// Detalhes da reserva (apenas para administradores)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_details: Option<ReservationDetails>,
}

/// Estrutura de resposta do endpoint do calendário
#[derive(Serialize)]
pub struct CalendarResponse {
    pub date: String,
    pub slots: Vec<TimeSlot>,
}

/// Endpoint GET /calendar?date=YYYY-MM-DD
/// Retorna um calendário para a data informada, com slots marcados como disponíveis ou indisponíveis.
pub async fn get_calendar(
    Extension(pool): Extension<Pool>,
    Query(query): Query<CalendarQuery>,
) -> Result<Json<CalendarResponse>, (StatusCode, String)> {
    // Parse da data fornecida
    let date = NaiveDate::parse_from_str(&query.date, "%Y-%m-%d")
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid date format. Use YYYY-MM-DD.".to_string()))?;
    
    // Defina o horário de funcionamento (por exemplo, 08:00 às 18:00) com intervalos de 30 minutos
    let start_time = NaiveTime::from_hms_opt(8, 0, 0).expect("Hora inicial inválida");
    let end_time = NaiveTime::from_hms_opt(18, 0, 0).expect("Hora final inválida");
    let slot_duration = ChronoDuration::minutes(30);
    
    let start_datetime = NaiveDateTime::new(date, start_time);
    let end_datetime = NaiveDateTime::new(date, end_time);
    
    // Obtenha a conexão com o banco e carregue as reservas do dia
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let day_reservations: Vec<Reservation> = reservations
        .filter(appointment_time.ge(start_datetime))
        .filter(appointment_time.lt(end_datetime))
        .load(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let mut slots = Vec::new();
    let mut current_time = start_datetime;
    
    // Itera sobre cada intervalo do dia
    while current_time < end_datetime {
        let slot_time = current_time.time().format("%H:%M").to_string();
        // Verifica se há uma reserva exatamente nesse horário
        let reservation_opt = day_reservations.iter().find(|r| r.appointment_time == current_time);
        
        // Defina status e, se necessário, os detalhes da reserva
        let (status, details) = if let Some(res) = reservation_opt {
            (
                "indisponível".to_string(), // Certifique-se de que é uma String
                Some(ReservationDetails {
                    client_name: res.client_name,
                    start_time: res.start_time,
                    end_time: res.end_time,
                }),
            )
        } else {
            ("disponível".to_string(), None) // Certifique-se de que é uma String
        };
        
        slots.push(TimeSlot {
            time: slot_time,
            status,
            reservation_details: details,
        });
        
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
