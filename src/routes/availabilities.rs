use axum::{Router, routing::{get, post}, Extension};
use std::sync::Arc;

use crate::{
    db::Pool,
    config::Config,
    handlers::availability::{
        create_availability, get_availabilities_for_professional
    },
};

pub fn router(pool: Pool, config: Arc<Config>) -> Router {
    Router::new()
        .route("/", post(create_availability))  // Rota para criação de disponibilidade
        .route("/:professional_id", get(get_availabilities_for_professional))  // Rota para pegar as disponibilidades de um profissional
        .layer(Extension(pool))  // Passando a Pool diretamente
        .layer(Extension(config)) // Passando a Config compartilhada
}
