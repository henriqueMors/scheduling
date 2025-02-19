use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    Router,
};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::db::Pool;
use crate::models::user::User;
use crate::schema::users::dsl::*;
use crate::services::auth_service::{verify_password, generate_sms_code, send_sms};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub phone: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    // Apenas para teste: em produção, não exponha o código SMS na resposta.
    pub sms_code: Option<String>,
}

/// Endpoint de login: valida telefone e senha, gera e "envia" o código SMS.
pub async fn login(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Busca o usuário pelo telefone
    let user: User = users.filter(phone.eq(&payload.phone))
        .first(&mut conn)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Credenciais inválidas".into()))?;
    
    // Verifica a senha
    if !verify_password(&user.password_hash, &payload.password) {
        return Err((StatusCode::UNAUTHORIZED, "Credenciais inválidas".into()));
    }
    
    // Gera e "envia" o código SMS
    let code = generate_sms_code();
    send_sms(&user.phone, &code);
    
    Ok(Json(LoginResponse {
        message: "Código SMS enviado. Verifique seu telefone.".into(),
        sms_code: Some(code), // Apenas para teste; remova em produção.
    }))
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub phone: String,
    pub sms_code: String,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    pub message: String,
    pub token: Option<String>, // Aqui você poderá gerar um token JWT para autenticar o usuário.
}

/// Endpoint de verificação: valida o código SMS e autentica o usuário.
pub async fn verify(
    Extension(_pool): Extension<Pool>,
    Json(payload): Json<VerifyRequest>,
) -> Result<Json<VerifyResponse>, (StatusCode, String)> {
    // Por simplicidade, se o código tiver 6 dígitos, consideramos-o válido.
    if payload.sms_code.len() == 6 {
        // Em uma implementação real, você atualizaria o campo sms_verified e geraria um token JWT.
        Ok(Json(VerifyResponse {
            message: "Usuário autenticado com sucesso!".into(),
            token: Some("jwt_token_exemplo".into()),
        }))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Código SMS inválido".into()))
    }
}

/// Agrega as rotas de autenticação.
pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/login", axum::routing::post(login))
        .route("/verify", axum::routing::post(verify))
        .layer(Extension(pool))
}
