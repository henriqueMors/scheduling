use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool as R2D2Pool};
use crate::config::Config;
use std::sync::Arc;

pub type Pool = R2D2Pool<ConnectionManager<PgConnection>>;

pub fn init_db(config: &Config) -> Arc<Pool> {
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    let pool = R2D2Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    Arc::new(pool) // Envolvendo o pool com Arc
}
