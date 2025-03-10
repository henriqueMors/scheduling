use std::env;
use dotenvy::dotenv;
use tracing::error;

/// 🔹 Estrutura para armazenar configurações do sistema.
#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub secret_key: String, // ✅ Adicionado secret_key
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        dotenv().ok(); // ✅ Carrega variáveis de ambiente automaticamente

        let database_url = env::var("DATABASE_URL").map_err(|_| {
            error!("❌ DATABASE_URL must be set in the environment");
            "DATABASE_URL must be set".to_string()
        })?;

        let secret_key = env::var("SECRET_KEY").map_err(|_| {
            error!("❌ SECRET_KEY must be set in the environment");
            "SECRET_KEY must be set".to_string()
        })?;

        Ok(Self {
            database_url,
            secret_key,
        })
    }
}
