use axum::{Router, routing::post, Extension, Json};
use crate::db::Pool;
use crate::services::admin_service;
use uuid::Uuid;

#[axum::debug_handler] // ✅ Corrigido erro do Axum Handler
async fn add_admin_handler(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<admin_service::NewAdmin>,
) -> Result<Json<admin_service::AdminResponse>, (axum::http::StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let admin = admin_service::add_admin(&mut conn, payload.master_id, payload.name, payload.phone, payload.password_hash)?;

    Ok(Json(admin))
}

/// Define as rotas do módulo `admin`
pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/add_admin", post(add_admin_handler))
        .layer(Extension(pool))
}
