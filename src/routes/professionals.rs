use axum::{Router, routing::{get, post, put, delete}, Extension};
use std::sync::Arc;

use crate::{
    db::Pool,
    config::Config,
    handlers::professional::{
        create_professional, get_professional_by_id, update_professional, delete_professional, list_professionals,
    },
};

pub fn router(pool: Arc<Pool>, config: Arc<Config>) -> Router { // Alterando para usar Arc<Pool>
    Router::new()
        .route("/", post(create_professional).get(list_professionals))  // Criar novo profissional e listar todos
        .route("/:id", get(get_professional_by_id))  // Buscar um profissional específico
        .route("/:id", put(update_professional))  // Atualizar um profissional específico
        .route("/:id", delete(delete_professional))  // Deletar um profissional específico
        .layer(Extension(pool))  // Passando a pool de conexões
        .layer(Extension(config))  // Passando a configuração do sistema
}
