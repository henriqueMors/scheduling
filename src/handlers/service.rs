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
    models::service::{Service, NewService, UpdateService},
    schema::services::dsl::*,
};

/// 🔹 Cria um novo serviço (somente admin)
pub async fn create_service(
    Extension(pool): Extension<Arc<Pool>>,  // Agora utilizando Arc<Pool>
    Json(payload): Json<NewService>,
) -> Result<Json<Service>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao obter conexão com o banco: {}", e))
    })?;

    let new_service = diesel::insert_into(services)
        .values(&payload)
        .get_result::<Service>(&mut conn)
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao criar serviço: {}", e))
        })?;

    Ok(Json(new_service))  // Retorna o serviço criado
}

/// 🔹 Lista todos os serviços
pub async fn list_services(
    Extension(pool): Extension<Arc<Pool>>,  // Agora utilizando Arc<Pool>
) -> Result<Json<Vec<Service>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao obter conexão com o banco: {}", e))
    })?;

    let results = services
        .filter(ativo.eq(true)) // Apenas serviços ativos
        .load::<Service>(&mut conn)
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao buscar serviços: {}", e))
        })?;

    Ok(Json(results))  // Retorna a lista de serviços
}

/// 🔹 Busca serviço por ID
pub async fn get_service_by_id(
    Extension(pool): Extension<Arc<Pool>>,  // Agora utilizando Arc<Pool>
    Path(service_id): Path<Uuid>,
) -> Result<Json<Service>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao obter conexão com o banco: {}", e))
    })?;

    let service = services
        .filter(id.eq(service_id))
        .first::<Service>(&mut conn)
        .map_err(|_| (StatusCode::NOT_FOUND, "Serviço não encontrado".to_string()))?;

    Ok(Json(service))  // Retorna o serviço encontrado
}

/// 🔹 Atualiza serviço
pub async fn update_service(
    Extension(pool): Extension<Arc<Pool>>,  // Agora utilizando Arc<Pool>
    Path(service_id): Path<Uuid>,
    Json(update): Json<UpdateService>,
) -> Result<Json<Service>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao obter conexão com o banco: {}", e))
    })?;

    let updated_service = diesel::update(services.filter(id.eq(service_id)))
        .set(update)
        .get_result::<Service>(&mut conn)
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao atualizar serviço: {}", e))
        })?;

    Ok(Json(updated_service))  // Retorna o serviço atualizado
}

/// 🔹 Deleta serviço
pub async fn delete_service(
    Extension(pool): Extension<Arc<Pool>>,  // Agora utilizando Arc<Pool>
    Path(service_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao obter conexão com o banco: {}", e))
    })?;

    diesel::delete(services.filter(id.eq(service_id)))
        .execute(&mut conn)
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao deletar serviço: {}", e))
        })?;

    Ok(StatusCode::NO_CONTENT)  // Retorna 204 (sem conteúdo)
}
