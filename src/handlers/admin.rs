use axum::{Router, routing::post, Extension, Json};
use crate::db::Pool;
use crate::services::admin_service::{self, NewAdmin, AdminResponse}; // ✅ Import corrigido

#[axum::debug_handler] // ✅ Evita erro de Handler no Axum
async fn add_admin_handler(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<NewAdmin>,
) -> Result<Json<AdminResponse>, (axum::http::StatusCode, String)> {
    let admin = admin_service::add_admin(payload.master_id, payload.name, payload.phone, payload.password_hash);
    Ok(Json(admin))
}

/// Define as rotas do módulo `admin`
pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/add_admin", post(add_admin_handler)) // ✅ Erro de `Handler` corrigido
        .layer(Extension(pool))
}
