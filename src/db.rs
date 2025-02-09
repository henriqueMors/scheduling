use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::PgConnection;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_db() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool")
}
