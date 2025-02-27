use std::env;
use dotenvy::dotenv;

/// Estrutura para armazenar configurações do sistema.
#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub secret_key: String, // ✅ Adicionado secret_key
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            secret_key: std::env::var("SECRET_KEY").expect("SECRET_KEY must be set"), // ✅ Certificando que existe
        }
    }
}


