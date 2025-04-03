use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    routing::{post, get, put, delete},
    Router,
};
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::{
    db::Pool,
    models::appointment::{Appointment, NewAppointment, UpdateAppointment},
    schema::appointments::dsl::*,
    schema::appointments,
};

/// 🔹 Cria um novo agendamento
pub async fn create_appointment(
    Extension(pool): Extension<Arc<Pool>>,  // Usando Arc<Pool> para garantir que a pool seja compartilhada
    Json(payload): Json<NewAppointment>,  // Recebendo dados de agendamento
) -> Result<Json<Appointment>, (StatusCode, String)> {
    // Obtendo conexão do pool
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Criando novo agendamento
    let new_appointment = diesel::insert_into(appointments)
        .values(&payload)
        .get_result::<Appointment>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(new_appointment))  // Retorna o agendamento criado
}

/// 🔹 Lista todos os agendamentos de um cliente
pub async fn list_appointments_by_client(
    Extension(pool): Extension<Arc<Pool>>,  // Usando Arc<Pool> para garantir que a pool seja compartilhada
    Path(client_id_from_path): Path<Uuid>,  // Mudando o nome da variável para evitar conflito
) -> Result<Json<Vec<Appointment>>, (StatusCode, String)> {
    // Obtendo conexão do pool
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Buscando todos os agendamentos para o cliente
    let appointments_list = appointments
        .filter(client_id.eq(client_id_from_path))  // Usando o client_id extraído do path
        .load::<Appointment>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(appointments_list))  // Retorna a lista de agendamentos
}

/// 🔹 Atualiza o status de um agendamento
pub async fn update_appointment(
    Extension(pool): Extension<Arc<Pool>>,  // Usando Arc<Pool> para garantir que a pool seja compartilhada
    Path(appointment_id): Path<Uuid>,  // Obtém o appointment_id a partir do path
    Json(update): Json<UpdateAppointment>,  // Dados para atualização
) -> Result<Json<Appointment>, (StatusCode, String)> {
    // Obtendo conexão do pool
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Atualizando o agendamento no banco
    let updated_appointment = diesel::update(appointments.filter(id.eq(appointment_id)))  // Filtra pelo ID
        .set(update)  // Atualiza com os dados recebidos
        .get_result::<Appointment>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(updated_appointment))  // Retorna o agendamento atualizado
}

/// 🔹 Deleta um agendamento
pub async fn delete_appointment(
    Extension(pool): Extension<Arc<Pool>>,  // Usando Arc<Pool> para garantir que a pool seja compartilhada
    Path(appointment_id): Path<Uuid>,  // Obtém o appointment_id a partir do path
) -> Result<StatusCode, (StatusCode, String)> {
    // Obtendo conexão do pool
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Deletando o agendamento do banco de dados
    diesel::delete(appointments.filter(id.eq(appointment_id)))  // Filtra pelo ID do agendamento
        .execute(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)  // Retorna o status de sucesso (204 No Content)
}
