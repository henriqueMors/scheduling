use axum::{Router, routing::post, Extension, Json};
use uuid::Uuid;
use crate::db::Pool;
use crate::services::auth_service::{hash_password};
use crate::models::user::{User, NewUser};

/// Endpoint de registro de usuário.
pub async fn register_user(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<NewUser>,
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let user = User {
        id: Uuid::new_v4(),
        name: payload.name,
        phone: payload.phone,
        password_hash: payload.password_hash,
        role: payload.role,
        sms_verified: payload.sms_verified,
    };

    Ok(Json(user)) // ✅ Retornando `User`, não `NewUser`
}

/// Define as rotas do módulo `auth`
pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .layer(Extension(pool))
}
