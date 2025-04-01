use axum::{
    Router,
    middleware::from_fn,
    routing::{get, put, delete, patch},
    Extension,
};
use std::sync::Arc;

use crate::{
    db::Pool,
    config::Config,
    middleware::auth_middleware::auth_middleware,
    handlers::user::{
        list_users,
        get_user_by_id,
        update_user,
        delete_user,
        update_user_role,
    },
};

pub fn router(pool: Pool, config: Arc<Config>) -> Router {
    Router::new()
        .route("/", get(list_users))
        .route("/:id", get(get_user_by_id).put(update_user).delete(delete_user))
        .route("/:id/role", patch(update_user_role))
        .layer(Extension(pool))
        .layer(Extension(config))
        .layer(from_fn(auth_middleware))
}
