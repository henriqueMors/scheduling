use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash};
use rand::rngs::OsRng;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use diesel::prelude::*;
use crate::config::Config;
use crate::models::user::User;
use crate::schema::users;

/// Estrutura para claims do JWT
#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

/// 🔹 Gera um token JWT para um usuário autenticado
pub fn generate_jwt(user: &User, config: &Config) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::new(0, 0))
        .as_secs() + 3600; // Token válido por 1 hora

    let claims = Claims {
        sub: user.id.to_string(),
        exp: expiration as usize,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(config.secret_key.as_ref()),
    )
}

/// 🔹 Hash de senha seguro com Argon2
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}

/// 🔹 Verifica se a senha fornecida corresponde ao hash armazenado
pub fn verify_password(hash: &str, password: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed_hash) => Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok(),
        Err(_) => false,
    }
}

/// 🔹 Valida um usuário pelo telefone e senha
pub fn validate_user(
    conn: &mut PgConnection,
    phone: &str,
    password: &str
) -> Result<User, String> {
    let user = users::table
        .filter(users::phone.eq(phone))
        .first::<User>(conn)
        .map_err(|_| "User not found".to_string())?;

    if verify_password(&user.password_hash, password) {
        Ok(user)
    } else {
        Err("Invalid password".to_string())
    }
}
