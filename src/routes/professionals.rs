use axum::{Router, routing::{get, post, put, delete}, Extension};
use std::sync::Arc;

use crate::{
    db::Pool,
    config::Config,
    handlers::professional::{
        create_professional, get_professional, update_professional, delete_professional, list_professionals,
    },
};

pub fn router(pool: Pool, config: Arc<Config>) -> Router {
    Router::new()
        .route("/", post(create_professional))  // Criar novo profissional
        .route("/", get(list_professionals))  // Listar todos os profissionais
        .route("/:id", get(get_professional))  // Buscar um profissional específico
        .route("/:id", put(update_professional))  // Atualizar um profissional específico
        .route("/:id", delete(delete_professional))  // Deletar um profissional específico
        .layer(Extension(pool))  // Passando a pool de conexões
        .layer(Extension(config))  // Passando a configuração do sistema
}
