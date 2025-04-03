use axum::{Router, routing::{get, post}, Extension};
use std::sync::Arc;

use crate::{
    db::Pool,
    config::Config,
    handlers::availability::{
        create_availability, list_availabilities_by_professional // Corrigido para o nome correto da função
    },
};

pub fn router(pool: Arc<Pool>, config: Arc<Config>) -> Router {
    Router::new()
        .route("/", post(create_availability))  // Rota para criação de disponibilidade
        .route("/:professional_id", get(list_availabilities_by_professional))  // Corrigido para o nome correto da função
        .layer(Extension(pool))  // Passando o Arc<Pool> diretamente
        .layer(Extension(config)) // Passando a Config compartilhada
}
