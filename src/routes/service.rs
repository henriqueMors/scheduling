use axum::{Router, routing::{get, post, put, delete}, Extension};
use std::sync::Arc;

use crate::{
    db::Pool,
    config::Config,
    handlers::service::{
        create_service, list_services, get_service_by_id, update_service, delete_service,
    },
};

pub fn router(pool: Pool, config: Arc<Config>) -> Router {
    Router::new()
        .route("/", post(create_service).get(list_services))
        .route("/:id", get(get_service_by_id).put(update_service).delete(delete_service))
        .layer(Extension(pool))
        .layer(Extension(config))
}
