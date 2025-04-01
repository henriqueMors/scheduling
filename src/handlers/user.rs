use axum::{
    extract::{Extension, Path, Json},
    http::StatusCode,
    routing::{get, put, delete, patch},
    Router,
};
use diesel::prelude::*;
use uuid::Uuid;
use std::sync::Arc;

use crate::{
    db::Pool,
    models::user::{User, UpdateUser},
    schema::users::dsl::*,
    middleware::auth_middleware::Claims,
};

/// 🔹 Lista todos os usuários (apenas admin)
pub async fn list_users(
    Extension(pool): Extension<Pool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    if claims.role != "admin" && claims.role != "admin_master" {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let mut conn = pool.get().map_err(internal_error)?;
    let results = users.load::<User>(&mut conn).map_err(internal_error)?;

    Ok(Json(results))
}

/// 🔹 Busca usuário por ID (autorizado ou admin)
pub async fn get_user_by_id(
    Extension(pool): Extension<Pool>,
    Extension(claims): Extension<Claims>,
    Path(target_id): Path<Uuid>,
) -> Result<Json<User>, (StatusCode, String)> {
    if claims.sub != target_id.to_string() && claims.role != "admin" && claims.role != "admin_master" {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let mut conn = pool.get().map_err(internal_error)?;
    let user_data = users
        .filter(id.eq(target_id))
        .first::<User>(&mut conn)
        .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(user_data))
}

/// 🔹 Atualiza dados do usuário (self ou admin)
pub async fn update_user(
    Extension(pool): Extension<Pool>,
    Extension(claims): Extension<Claims>,
    Path(target_id): Path<Uuid>,
    Json(update): Json<UpdateUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    if claims.sub != target_id.to_string() && claims.role != "admin" && claims.role != "admin_master" {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let mut conn = pool.get().map_err(internal_error)?;

    let updated = diesel::update(users.filter(id.eq(target_id)))
        .set(update)
        .get_result::<User>(&mut conn)
        .map_err(internal_error)?;

    Ok(Json(updated))
}

/// 🔹 Deleta um usuário (self ou admin)
pub async fn delete_user(
    Extension(pool): Extension<Pool>,
    Extension(claims): Extension<Claims>,
    Path(target_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    if claims.sub != target_id.to_string() && claims.role != "admin" && claims.role != "admin_master" {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let mut conn = pool.get().map_err(internal_error)?;

    diesel::delete(users.filter(id.eq(target_id)))
        .execute(&mut conn)
        .map_err(internal_error)?;

    Ok(StatusCode::NO_CONTENT)
}

/// 🔹 Admin atualiza o `role` de um usuário
#[derive(serde::Deserialize)]
pub struct RoleUpdate {
    pub role: String,
}

pub async fn update_user_role(
    Extension(pool): Extension<Pool>,
    Extension(claims): Extension<Claims>,
    Path(target_id): Path<Uuid>,
    Json(body): Json<RoleUpdate>,
) -> Result<Json<User>, (StatusCode, String)> {
    if claims.role != "admin" && claims.role != "admin_master" {
        return Err((StatusCode::FORBIDDEN, "Only admin can update roles".to_string()));
    }

    let mut conn = pool.get().map_err(internal_error)?;

    let updated = diesel::update(users.filter(id.eq(target_id)))
        .set(role.eq(body.role))
        .get_result::<User>(&mut conn)
        .map_err(internal_error)?;

    Ok(Json(updated))
}

// 🔧 Utilitário para converter erros internos
fn internal_error<E: std::fmt::Debug>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err))
}
