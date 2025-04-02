use axum::{Router, routing::{get, post, put, delete}, Extension};
use std::sync::Arc;

use crate::{
    db::Pool,
    config::Config,
    handlers::appointment::{
        create_appointment, list_appointments_by_client, update_appointment, delete_appointment,
    },
};

pub fn router(pool: Pool, config: Arc<Config>) -> Router {
    Router::new()
        // Rota para criar um novo agendamento
        .route("/", post(create_appointment))

        // Rota para listar agendamentos de um cliente, usando client_id
        .route("/client/:client_id", get(list_appointments_by_client))  // Mudamos o caminho para /client/:client_id

        // Rota para atualizar ou deletar um agendamento usando o ID do agendamento
        .route("/:id", put(update_appointment).delete(delete_appointment))  // Para agendamentos específicos, usamos o ID
        .layer(Extension(pool))
        .layer(Extension(config))
}
