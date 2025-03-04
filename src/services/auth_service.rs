use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash};
use rand::rngs::OsRng;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use crate::config::Config;
use crate::models::user::User;
use serde::{Serialize, Deserialize};

/// 🔹 Estrutura para os "claims" do token JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // ID do usuário
    pub exp: usize,   // Timestamp de expiração
    pub role: String, // Papel do usuário ("client", "admin", "admin_master")
}

/// 🔐 Gera um token JWT para um usuário autenticado
pub fn generate_jwt(user: &User, config: &Config) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .checked_add(Duration::from_secs(3600)) // Token válido por 1 hora
        .expect("Erro ao calcular expiração")
        .duration_since(UNIX_EPOCH)
        .expect("Erro ao calcular tempo")
        .as_secs() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        exp: expiration,
        role: user.role.clone(),  // ✅ Inclui o papel do usuário no token
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(config.secret_key.as_bytes()),
    )
}

/// ✅ Valida um token JWT e retorna os claims do usuário autenticado
pub fn validate_jwt(token: &str, config: &Config) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret_key.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(decoded.claims) // ✅ Retorna a estrutura completa de Claims (incluindo role)
}

/// 🔐 Hash da senha ao cadastrar
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}

/// 🔐 Verifica se a senha está correta
pub fn verify_password(hash: &str, password: &str) -> bool {
    if let Ok(parsed_hash) = PasswordHash::new(hash) {
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
    } else {
        false
    }
}
