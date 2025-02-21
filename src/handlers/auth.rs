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
use crate::services::auth_service::{verify_password, generate_sms_code, send_sms, hash_password};
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};
use std::time::{SystemTime, Duration};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub phone: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    // Apenas para teste; em produção, remova o sms_code da resposta.
    pub sms_code: Option<String>,
}

/// Struct para as Claims do JWT.
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // ID do usuário
    pub exp: usize,   // Expiração (em segundos)
}

/// Endpoint de login: valida telefone e senha, gera e "envia" o código SMS.
pub async fn login(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Busca o usuário pelo telefone, certificando-se de que o role seja "client"
    let user: User = users.filter(phone.eq(&payload.phone))
        .filter(role.eq("client"))
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
    pub token: Option<String>, // Token JWT
}

/// Gera o token JWT para o usuário autenticado.
fn generate_jwt(user: &User) -> String {
    let expiration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap() + Duration::new(3600, 0); // Token expira em 1 hora

    let claims = Claims {
        sub: user.id.to_string(),
        exp: expiration.as_secs() as usize,
    };

    encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret("secret_key".as_ref())).unwrap()
}

/// Endpoint de verificação: valida o código SMS e autentica o usuário.
pub async fn verify(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<VerifyRequest>,
) -> Result<Json<VerifyResponse>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Busca o usuário pelo telefone
    let user: User = users.filter(phone.eq(&payload.phone))
        .first(&mut conn)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Usuário não encontrado".into()))?;

    // Aqui você validaria o código SMS (exemplo simples: se tiver 6 dígitos, é válido)
    if payload.sms_code.len() == 6 {
        // Em uma implementação real, você atualizaria o campo sms_verified do usuário.
        let token = generate_jwt(&user);
        Ok(Json(VerifyResponse {
            message: "Usuário autenticado com sucesso!".into(),
            token: Some(token),
        }))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Código SMS inválido".into()))
    }
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub phone: String,
    pub current_password: String,
    pub new_password: String,
}

#[derive(Serialize)]
pub struct ChangePasswordResponse {
    pub message: String,
}

/// Endpoint de troca de senha: valida a senha atual e atualiza com a nova senha.
pub async fn change_password(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Json<ChangePasswordResponse>, (StatusCode, String)> {
    use crate::schema::users::dsl::*;
    
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Busca o usuário pelo telefone.
    let user: User = users.filter(phone.eq(&payload.phone))
        .first(&mut conn)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "User not found".into()))?;
    
    // Verifica se a senha atual está correta.
    if !verify_password(&user.password_hash, &payload.current_password) {
        return Err((StatusCode::UNAUTHORIZED, "Invalid current password".into()));
    }
    
    // Gera o hash da nova senha.
    let new_hash = hash_password(&payload.new_password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Atualiza o campo password_hash do usuário.
    diesel::update(users.filter(id.eq(user.id)))
        .set(password_hash.eq(new_hash))
        .execute(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(ChangePasswordResponse {
        message: "Password updated successfully".into(),
    }))
}

/// Agrega todas as rotas de autenticação.
pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/login", axum::routing::post(login))
        .route("/verify", axum::routing::post(verify))
        .route("/change_password", axum::routing::post(change_password))
        .layer(Extension(pool))
}
