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
    Extension(pool): Extension<Pool>,
    Json(payload): Json<NewSalonSetting>,
) -> Result<Json<SalonSetting>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let new_setting = diesel::insert_into(salon_settings)
        .values(&payload)
        .get_result::<SalonSetting>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(new_setting))
}

/// 🔹 Lista a configuração atual do salão
pub async fn get_salon_setting(
    Extension(pool): Extension<Pool>,
) -> Result<Json<SalonSetting>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let setting = salon_settings
        .first::<SalonSetting>(&mut conn)
        .map_err(|_| (StatusCode::NOT_FOUND, "Salon settings not found".to_string()))?;

    Ok(Json(setting))
}

/// 🔹 Atualiza a configuração do salão
pub async fn update_salon_setting(
    Extension(pool): Extension<Pool>,
    Path(id): Path<Uuid>,
    Json(update): Json<UpdateSalonSetting>,
) -> Result<Json<SalonSetting>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let updated_setting = diesel::update(salon_settings.filter(id.eq(id)))
        .set(update)
        .get_result::<SalonSetting>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(updated_setting))
}
