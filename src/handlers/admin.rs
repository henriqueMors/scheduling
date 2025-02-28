use axum::{Router, routing::post, Extension, Json, http::StatusCode};
use diesel::r2d2::{Pool, ConnectionManager, PooledConnection};
use diesel::PgConnection;

use crate::db::Pool as DbPool;
use crate::models::admin::NewAdmin;
use crate::services::admin_service::{self, AdminResponse};

#[axum::debug_handler] // ✅ Evita erro de `Handler` no Axum
async fn add_admin_handler(
    Extension(pool): Extension<DbPool>,
    Json(payload): Json<NewAdmin>,
) -> Result<Json<AdminResponse>, (StatusCode, String)> {
    // Obtendo conexão do pool
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Chamando o serviço de adição de admin
    match admin_service::add_admin(&mut conn, payload) {
        Ok(admin) => Ok(Json(admin)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Define as rotas do módulo `admin`
pub fn router(pool: DbPool) -> Router {
    Router::new()
        .route("/add_admin", post(add_admin_handler))
        .layer(Extension(pool))
}
