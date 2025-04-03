use axum::{Router, routing::{get, post, put}, Extension};
use std::sync::Arc;

use crate::{
    db::Pool,
    config::Config,
    handlers::salon_settings::{create_salon_setting, get_salon_setting, update_salon_setting},
};

pub fn router(pool: Arc<Pool>, config: Arc<Config>) -> Router {
    Router::new()
        .route("/", post(create_salon_setting)) // Criar configuração
        .route("/", get(get_salon_setting))    // Obter configuração
        .route("/:id", put(update_salon_setting)) // Atualizar configuração
        .layer(Extension(pool))  // Compartilhar a conexão com o banco
        .layer(Extension(config)) // Compartilhar a configuração
}
