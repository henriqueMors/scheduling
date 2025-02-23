use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    Router,
};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::db::Pool;
use crate::models::user::{User, NewUser};
use crate::schema::users::dsl::*;
use crate::services::auth_service::{hash_password, verify_password, generate_sms_code, send_sms};
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm, DecodingKey, Validation, decode};
use std::time::{SystemTime, Duration};
use uuid::Uuid;

//
// REGISTRATION
//

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub phone: String,
    pub password: String,
    pub role: Option<String>, // Se enviado, usará esse valor; se não, usará "client" por padrão.
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub message: String,
}

/// Endpoint de registro: cria um novo usuário com role "client".
pub async fn register(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Verifica se já existe um usuário com o mesmo telefone.
    let existing: Result<User, _> = users.filter(phone.eq(&payload.phone)).first(&mut conn);
    if existing.is_ok() {
        return Err((StatusCode::CONFLICT, "User already exists".into()));
    }
    
    // Gera o hash da senha (renomeado para evitar conflito com a coluna do schema).
    let new_password_hash = hash_password(&payload.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Define o role: se o payload incluir um role, usa-o; caso contrário, define como "client".
    let role_value = payload.role.unwrap_or_else(|| "client".to_string());

    // Cria o novo usuário com role "client" e sms_verified como false.
    let new_user = NewUser {
        name: payload.name,
        phone: payload.phone,
        password_hash: new_password_hash,
        role: "client".to_string(),
        sms_verified: false,
    };
    
    // Insere o novo usuário no banco.
    let inserted: User = diesel::insert_into(users)
        .values(&new_user)
        .get_result(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(RegisterResponse {
        message: format!("User {} registered successfully.", inserted.name),
    }))
}

//
// LOGIN & VERIFICATION (Geração de código SMS e JWT)
//

#[derive(Deserialize)]
pub struct LoginRequest {
    pub phone: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    // Apenas para teste; em produção, o código SMS não deve ser retornado.
    pub sms_code: Option<String>,
}

/// Endpoint de login: valida telefone e senha, gera e "envia" o código SMS.
pub async fn login(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Busca o usuário pelo telefone, garantindo que o role seja "client"
    let user: User = users
        .filter(phone.eq(&payload.phone))
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
    pub token: Option<String>, // Token JWT gerado
}

/// Gera um token JWT para o usuário autenticado.
fn generate_jwt(user: &User) -> String {
    let expiration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap() + Duration::new(3600, 0); // Token expira em 1 hora

    #[derive(Serialize, Deserialize)]
    struct Claims {
        sub: String, // ID do usuário
        exp: usize,  // Expiração (em segundos desde epoch)
    }

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
    
    // Busca o usuário pelo telefone.
    let user: User = users
        .filter(phone.eq(&payload.phone))
        .first(&mut conn)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Usuário não encontrado".into()))?;
    
    // Exemplo simples de validação: se o código tiver 6 dígitos, considera válido.
    if payload.sms_code.len() == 6 {
        let token = generate_jwt(&user);
        Ok(Json(VerifyResponse {
            message: "Usuário autenticado com sucesso!".into(),
            token: Some(token),
        }))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Código SMS inválido".into()))
    }
}

//
// TROCA DE SENHA (usuário autenticado muda sua senha)
//

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
    let user: User = users
        .filter(phone.eq(&payload.phone))
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

//
// RECUPERAÇÃO DE SENHA: Esqueci a senha / Reset de senha
//

#[derive(Deserialize)]
pub struct ForgotPasswordRequest {
    pub phone: String,
}

#[derive(Serialize)]
pub struct ForgotPasswordResponse {
    pub message: String,
    // Apenas para testes, retorna o token; em produção, não retorne o token na resposta.
    pub token: Option<String>,
}

/// Claims específicos para o reset de senha (válido por 5 minutos)
#[derive(Serialize, Deserialize)]
pub struct ResetClaims {
    pub sub: String,   // ID do usuário
    pub exp: usize,    // Expiração (5 minutos)
    pub reset: bool,   // Flag para indicar que este token é para reset de senha
}

/// Endpoint para solicitar o reset de senha: gera um token temporário e o envia via SMS.
pub async fn forgot_password(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<Json<ForgotPasswordResponse>, (StatusCode, String)> {
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Busca o usuário pelo telefone.
    let user: User = users
        .filter(phone.eq(&payload.phone))
        .first(&mut conn)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "User not found".into()))?;
    
    // Define a expiração para 5 minutos.
    let expiration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap() + Duration::new(300, 0);
    
    let claims = ResetClaims {
        sub: user.id.to_string(),
        exp: expiration.as_secs() as usize,
        reset: true,
    };
    
    // Gera o token JWT para reset.
    let token = encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret("secret_reset_key".as_ref()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Simula o envio do token por SMS (aqui, apenas imprime no console).
    println!("Enviando SMS para {}: seu token de reset é {}", user.phone, token);
    
    Ok(Json(ForgotPasswordResponse {
        message: "Password reset token sent via SMS.".into(),
        token: Some(token), // Apenas para teste; remova em produção.
    }))
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    pub phone: String,
    pub new_password: String,
    pub token: String,
}

#[derive(Serialize)]
pub struct ResetPasswordResponse {
    pub message: String,
}

/// Endpoint para redefinir a senha utilizando o token temporário.
pub async fn reset_password(
    Extension(pool): Extension<Pool>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<Json<ResetPasswordResponse>, (StatusCode, String)> {
    let validation = Validation {
        algorithms: vec![Algorithm::HS256],
        ..Validation::default()
    };
    
    let token_data = decode::<ResetClaims>(
        &payload.token,
        &DecodingKey::from_secret("secret_reset_key".as_ref()),
        &validation,
    ).map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid token: {}", e)))?;
    
    let user_id = Uuid::parse_str(&token_data.claims.sub)
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;
    
    if !token_data.claims.reset {
        return Err((StatusCode::UNAUTHORIZED, "Token is not valid for password reset".into()));
    }
    
    let mut conn = pool.get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Busca o usuário pelo ID e verifica se o telefone confere.
    let user: User = users
        .filter(id.eq(user_id))
        .first(&mut conn)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "User not found".into()))?;
    
    if user.phone != payload.phone {
        return Err((StatusCode::UNAUTHORIZED, "Phone does not match".into()));
    }
    
    let new_hash = hash_password(&payload.new_password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    diesel::update(users.filter(id.eq(user_id)))
        .set(password_hash.eq(new_hash))
        .execute(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(ResetPasswordResponse {
        message: "Password reset successfully.".into(),
    }))
}

//
// Aggregador das rotas de autenticação
//
pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/register", axum::routing::post(register))
        .route("/login", axum::routing::post(login))
        .route("/verify", axum::routing::post(verify))
        .route("/change_password", axum::routing::post(change_password))
        .route("/forgot_password", axum::routing::post(forgot_password))
        .route("/reset_password", axum::routing::post(reset_password))
        .layer(Extension(pool))
}
