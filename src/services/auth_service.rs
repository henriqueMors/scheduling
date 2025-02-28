use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash};
use rand::rngs::OsRng;
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};
use crate::config::Config;
use crate::models::user::User;

/// 🔹 Estrutura para claims do JWT
#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

/// 🔹 Gera um token JWT
pub fn generate_jwt(user: &User, config: &Config) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        sub: user.id.to_string(),
        exp: (chrono::Utc::now().timestamp() + 3600) as usize,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(config.secret_key.as_ref()),
    )
}

/// 🔹 Hash de senha com Argon2
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default().hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}

/// 🔹 Verifica a senha
pub fn verify_password(hash: &str, password: &str) -> bool {
    if let Ok(parsed_hash) = PasswordHash::new(hash) {
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
    } else {
        false
    }
}
