use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool as R2D2Pool};
use crate::config::Config;
use tracing::error;

pub type Pool = R2D2Pool<ConnectionManager<PgConnection>>;

/// 🔹 Inicializa o pool de conexões com logs em caso de erro
pub fn init_db(config: &Config) -> Result<Pool, String> {
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    R2D2Pool::builder()
        .build(manager)
        .map_err(|e| {
            error!("❌ Failed to create pool: {}", e);
            format!("Failed to create pool: {}", e)
        })
}
