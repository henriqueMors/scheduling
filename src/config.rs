use std::env;
use dotenvy::dotenv;

/// Estrutura para armazenar configurações do sistema.
#[derive(Clone)] // ✅ Adicionado Clone para evitar erro ao usar Extension
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }
}

