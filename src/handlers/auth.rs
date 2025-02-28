use axum::{Router, routing::post, Extension, Json, http::StatusCode};
use diesel::prelude::*;
use uuid::Uuid;
use crate::db::Pool;
use crate::services::auth_service::{hash_password, validate_user, generate_jwt};
use crate::models::user::{User, NewUser, InsertableUser, LoginRequest, LoginResponse};
use crate::schema::users;

/// 🔹 Registro de usuário
pub async fn register_user(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let hashed_password = hash_password(&payload.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let new_user = InsertableUser {
        id: Uuid::new_v4(),
        name: payload.name,
        phone: payload.phone,
        password_hash: hashed_password,
        role: payload.role,
        sms_verified: payload.sms_verified,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let saved_user = users::table
        .filter(users::phone.eq(&new_user.phone))
        .first::<User>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(saved_user))
}

/// 🔹 Login de usuário
pub async fn login_user(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match validate_user(&mut conn, &payload.phone, &payload.password) {
        Ok(user) => {
            let token = generate_jwt(&user).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            Ok(Json(LoginResponse { token }))
        }
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid credentials".into())),
    }
}

/// 🔹 Define as rotas do módulo `auth`
pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user)) // ✅ Adicionando login
        .layer(Extension(pool))
}
