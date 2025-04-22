use axum::{Router, routing::{get, post, put, delete}, Extension};
use std::sync::Arc;

use crate::{
    db::Pool,
    config::Config,
    handlers::appointment::{
        create_appointment, list_appointments_by_client, update_appointment, delete_appointment,
    },
};

pub fn router(pool: Arc<Pool>, config: Arc<Config>) -> Router {
    Router::new()
        .route("/", post(create_appointment))
        .route("/client/:client_id", get(list_appointments_by_client))  // Modificado
        .route("/appointment/:id", put(update_appointment).delete(delete_appointment))  // Modificado
        .layer(Extension(pool))
        .layer(Extension(config))
}
