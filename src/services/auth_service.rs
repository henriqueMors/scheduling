use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash};
use rand::rngs::OsRng;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use std::time::{SystemTime, Duration};
use crate::config::Config;
use crate::models::user::User;

/// Estrutura para claims do JWT
#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_jwt(config: &Config) -> String {
    let claims = serde_json::json!({
        "sub": "user_id",
        "exp": chrono::Utc::now().timestamp() + 3600
    });

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(config.secret_key.as_ref()), // ✅ Agora existe `secret_key`
    )
    .unwrap()
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    if let Ok(parsed_hash) = PasswordHash::new(hash) {
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
    } else {
        false
    }
}
