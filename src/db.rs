use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool as R2D2Pool};

pub type Pool = R2D2Pool<ConnectionManager<PgConnection>>;

pub fn init_db() -> Pool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    R2D2Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
