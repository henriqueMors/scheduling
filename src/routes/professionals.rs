use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post, put, delete},
    Extension,
};
use std::sync::Arc;

use crate::{
    db::Pool,
    config::Config,
    middleware::auth_middleware::{auth_middleware},
    handlers::professional::{
        create_professional, list_professionals, get_professional_by_id,
        update_professional, delete_professional,
    },
};

pub fn router(pool: Pool, config: Arc<Config>) -> Router {
    Router::new()
        .route("/", post(create_professional).get(list_professionals))
        .route("/:id", get(get_professional_by_id).put(update_professional).delete(delete_professional))
        .layer(Extension(pool))
        .layer(Extension(config))
        .layer(from_fn(auth_middleware)) // 🔐 Proteção JWT
}
