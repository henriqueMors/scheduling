use axum::{
    Router, routing::{get, post, delete},
    Extension, Json, extract::Path,
    http::StatusCode,
};
use uuid::Uuid;
use crate::db::Pool;
use crate::services::admin_service::{self, AdminResponse};
use crate::models::admin::NewAdmin;


/// 🔹 Adiciona um administrador.
#[axum::debug_handler]
async fn add_admin_handler(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<NewAdmin>,
) -> Result<Json<AdminResponse>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let admin = admin_service::add_admin(&mut conn, payload)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(admin))
}

/// 🔹 Lista todos os administradores.
#[axum::debug_handler]
async fn list_admins_handler(
    Extension(pool): Extension<Pool>,
) -> Result<Json<Vec<AdminResponse>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let admins = admin_service::list_admins(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(admins))
}

/// 🔹 Remove um administrador pelo ID.
#[axum::debug_handler]
async fn remove_admin_handler(
    Extension(pool): Extension<Pool>,
    Path(admin_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match admin_service::remove_admin(&mut conn, admin_id) {
        Ok(deleted) if deleted > 0 => Ok(Json(serde_json::json!({"message": "Admin deleted"}))),
        Ok(_) => Err((StatusCode::NOT_FOUND, "Admin not found".into())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// 🔹 Define as rotas do módulo `admin`.
pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/", get(list_admins_handler))  // ✅ Rota para listar admins
        .route("/add_admin", post(add_admin_handler))  // ✅ Rota para adicionar admin
        .route("/{admin_id}", delete(remove_admin_handler))  // ✅ Rota para remover admin
        .layer(Extension(pool))
}
