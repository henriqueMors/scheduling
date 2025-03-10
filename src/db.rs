use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool as R2D2Pool};
use crate::config::Config;

pub type Pool = R2D2Pool<ConnectionManager<PgConnection>>;

pub fn init_db(config: &Config) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    R2D2Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
