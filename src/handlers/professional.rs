use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    routing::{get, post, put, delete},
    Router,
};
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    db::Pool,
    models::professional::{Professional, NewProfessional, UpdateProfessional},
    schema::professionals::dsl::*,
    middleware::auth_middleware::{Claims, require_role},
};

/// 🔹 Cria um novo profissional (somente admin)
pub async fn create_professional(
    Extension(pool): Extension<Pool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<NewProfessional>,
) -> Result<Json<Professional>, (StatusCode, String)> {
    if claims.role != "admin" && claims.role != "admin_master" {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let inserted = diesel::insert_into(professionals)
        .values(&payload)
        .get_result::<Professional>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(inserted))
}

/// 🔹 Lista todos os profissionais
pub async fn list_professionals(
    Extension(pool): Extension<Pool>,
) -> Result<Json<Vec<Professional>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let results = professionals
        .order(created_at.desc())
        .load::<Professional>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(results))
}

/// 🔹 Busca profissional por ID
pub async fn get_professional_by_id(
    Extension(pool): Extension<Pool>,
    Path(prof_id): Path<Uuid>,
) -> Result<Json<Professional>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let result = professionals
        .filter(id.eq(prof_id))
        .first::<Professional>(&mut conn)
        .map_err(|_| (StatusCode::NOT_FOUND, "Professional not found".to_string()))?;

    Ok(Json(result))
}

/// 🔹 Atualiza um profissional
pub async fn update_professional(
    Extension(pool): Extension<Pool>,
    Path(prof_id): Path<Uuid>,
    Json(update): Json<UpdateProfessional>,
) -> Result<Json<Professional>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let updated = diesel::update(professionals.filter(id.eq(prof_id)))
        .set(&update)
        .get_result::<Professional>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(updated))
}

/// 🔹 Deleta um profissional
pub async fn delete_professional(
    Extension(pool): Extension<Pool>,
    Path(prof_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    diesel::delete(professionals.filter(id.eq(prof_id)))
        .execute(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
