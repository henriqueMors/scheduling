use crate::schema::users::dsl::*;

use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    Router,
};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::db::Pool;
use crate::models::user::User;
use crate::services::auth_service::{verify_password, generate_sms_code, send_sms};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::time::{SystemTime, Duration};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub phone: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub sms_code: Option<String>, // Apenas para teste: em produção, remova.
}

/// Endpoint de login: valida telefone e senha, gera e "envia" o código SMS.
pub async fn login(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Busca o usuário pelo telefone, certificando-se que o role seja "client" ou "admin".
    let user: User = users.filter(phone.eq(&payload.phone))
        .filter(role.eq("client"))  // Para login de clientes
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
    pub token: Option<String>, // Geração do token JWT
}

/// Gera um token JWT para o usuário autenticado.
fn generate_jwt(user: &User) -> String {
    let expiration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        + Duration::new(3600, 0);  // O token expira em 1 hora

    let claims = jsonwebtoken::Claims {
        sub: user.id.to_string(),
        exp: expiration.as_secs() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret("secret_key".as_ref())).unwrap()
}

/// Endpoint de verificação: valida o código SMS e autentica o usuário.
pub async fn verify(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<VerifyRequest>,
) -> Result<Json<VerifyResponse>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Busca o usuário pelo telefone
    let user: User = users.filter(phone.eq(&payload.phone))
        .first(&mut conn)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Usuário não encontrado".into()))?;

    // Aqui você validaria o código SMS (exemplo simples de verificação)
    if payload.sms_code.len() == 6 {
        // Atualize o campo sms_verified do usuário (em uma implementação real)
        
        // Gere o JWT e retorne ao usuário
        let token = generate_jwt(&user);
        Ok(Json(VerifyResponse {
            message: "Usuário autenticado com sucesso!".into(),
            token: Some(token),
        }))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Código SMS inválido".into()))
    }
}

/// Agrega as rotas de autenticação
pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/login", axum::routing::post(login))
        .route("/verify", axum::routing::post(verify))
        .layer(Extension(pool))
}
