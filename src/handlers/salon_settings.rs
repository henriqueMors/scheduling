use axum::{extract::{Extension, Json, Path}, http::StatusCode};
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    db::Pool,
    models::salon_settings::{SalonSetting, NewSalonSetting, UpdateSalonSetting},
    schema::salon_settings::dsl::*,
};

/// 🔹 Cria uma nova configuração para o salão
pub async fn create_salon_setting(
    Extension(pool): Extension<Arc<Pool>>,  // Agora recebendo Arc<Pool>
    Json(payload): Json<NewSalonSetting>,
) -> Result<Json<SalonSetting>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao obter conexão: {}", e))
    })?;

    let new_setting = diesel::insert_into(salon_settings)
        .values(&payload)
        .get_result::<SalonSetting>(&mut conn)
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao criar configuração: {}", e))
        })?;

    Ok(Json(new_setting))
}

/// 🔹 Lista a configuração atual do salão
pub async fn get_salon_setting(
    Extension(pool): Extension<Arc<Pool>>,  // Agora recebendo Arc<Pool>
) -> Result<Json<SalonSetting>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao obter conexão: {}", e))
    })?;

    let setting = salon_settings
        .first::<SalonSetting>(&mut conn)
        .map_err(|_| (StatusCode::NOT_FOUND, "Configuração do salão não encontrada".to_string()))?;

    Ok(Json(setting))
}

/// 🔹 Atualiza a configuração do salão
pub async fn update_salon_setting(
    Extension(pool): Extension<Arc<Pool>>,  // Agora recebendo Arc<Pool>
    Path(id): Path<Uuid>,  // Extraindo o id do salão a partir da URL
    Json(update): Json<UpdateSalonSetting>,
) -> Result<Json<SalonSetting>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao obter conexão: {}", e))
    })?;

    let updated_setting = diesel::update(salon_settings.filter(id.eq(id)))  // Usando o id extraído do Path
        .set(update)
        .get_result::<SalonSetting>(&mut conn)
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao atualizar configuração: {}", e))
        })?;

    Ok(Json(updated_setting))
}
