use std::env;
use dotenvy::dotenv;

/// Estrutura para armazenar configurações do sistema.
pub struct Config {
    pub database_url: String,
    pub secret_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            secret_key: env::var("SECRET_KEY").expect("SECRET_KEY must be set"),
        }
    }
}
