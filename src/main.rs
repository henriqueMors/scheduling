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
mod config;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = config::Config::from_env();
    let pool = db::init_db(&config);

    let app = Router::new()
    .nest("/clients", routes::clients::router(pool.clone()))
    .nest("/reservations", routes::reservations::router(pool.clone()))
    .nest("/auth", handlers::auth::router(pool.clone(), config.clone()))
    .nest("/admin", handlers::admin::router(pool.clone()))
    .layer(Extension(pool))
    .layer(Extension(config));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Servidor rodando em http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
