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
};

/// 🔹 Cria um novo profissional (somente admin)
pub async fn create_professional(
    Extension(pool): Extension<Arc<Pool>>,  // Passando Arc<Pool> para que o pool seja compartilhado corretamente
    Json(payload): Json<NewProfessional>,  // Dados do novo profissional
) -> Result<Json<Professional>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao obter conexão: {}", e))
    })?;

    let inserted = diesel::insert_into(professionals)
        .values(&payload)
        .get_result::<Professional>(&mut conn)
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao criar profissional: {}", e))
        })?;

    Ok(Json(inserted))
}

/// 🔹 Lista todos os profissionais
pub async fn list_professionals(
    Extension(pool): Extension<Arc<Pool>>,
) -> Result<Json<Vec<Professional>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao obter conexão: {}", e))
    })?;

    let results = professionals
        .order(created_at.desc())
        .load::<Professional>(&mut conn)
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao listar profissionais: {}", e))
        })?;

    Ok(Json(results))
}

/// 🔹 Busca um profissional específico pelo ID
pub async fn get_professional_by_id(
    Extension(pool): Extension<Arc<Pool>>,
    Path(prof_id): Path<Uuid>,
) -> Result<Json<Professional>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao obter conexão: {}", e))
    })?;

    let result = professionals
        .filter(id.eq(prof_id))
        .first::<Professional>(&mut conn)
        .map_err(|_| (StatusCode::NOT_FOUND, "Profissional não encontrado".to_string()))?;

    Ok(Json(result))
}

/// 🔹 Atualiza um profissional
pub async fn update_professional(
    Extension(pool): Extension<Arc<Pool>>,
    Path(prof_id): Path<Uuid>,
    Json(update): Json<UpdateProfessional>,
) -> Result<Json<Professional>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao obter conexão: {}", e))
    })?;

    let updated = diesel::update(professionals.filter(id.eq(prof_id)))
        .set(&update)
        .get_result::<Professional>(&mut conn)
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao atualizar profissional: {}", e))
        })?;

    Ok(Json(updated))
}

/// 🔹 Deleta um profissional
pub async fn delete_professional(
    Extension(pool): Extension<Arc<Pool>>,
    Path(prof_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao obter conexão: {}", e))
    })?;

    diesel::delete(professionals.filter(id.eq(prof_id)))
        .execute(&mut conn)
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao deletar profissional: {}", e))
        })?;

    Ok(StatusCode::NO_CONTENT)
}
