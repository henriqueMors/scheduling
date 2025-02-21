use axum::{Router, Extension};
use std::net::SocketAddr;
use dotenvy::dotenv;
use tokio::net::TcpListener;

mod db;
mod models;
mod handlers;
mod routes;
mod services;
mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = db::init_db();

    let app = Router::new()
        .nest("/clients", routes::clients::router(pool.clone()))
        .nest("/reservations", routes::reservations::router(pool.clone()))
        .nest("/auth", handlers::auth::router(pool.clone()))
        .nest("/admin", handlers::admin::admin_router(pool.clone()))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
