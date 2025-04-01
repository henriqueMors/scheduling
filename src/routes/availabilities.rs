use axum::{Router, routing::{get, post, put, delete}, Extension};
use std::sync::Arc;

use crate::{
    db::Pool,
    config::Config,
    handlers::availability::{
        create_availability, list_availabilities_by_professional, update_availability, delete_availability,
    },
};

pub fn router(pool: Pool, config: Arc<Config>) -> Router {
    Router::new()
        .route("/", post(create_availability))
        .route("/:professional_id", get(list_availabilities_by_professional))
        .route("/:id", put(update_availability).delete(delete_availability))
        .layer(Extension(pool))
        .layer(Extension(config))
}
