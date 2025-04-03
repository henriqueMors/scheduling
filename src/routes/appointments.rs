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
        .route("/:client_id", get(list_appointments_by_client))
        .route("/:id", put(update_appointment).delete(delete_appointment))
        .layer(Extension(pool))   // Aqui, passamos o Arc<Pool>
        .layer(Extension(config)) // Passamos o Config normalmente
}
