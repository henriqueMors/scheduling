use axum::{Router, Extension, middleware};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use crate::middleware::auth_middleware::auth_middleware; // ✅ Importando o middleware

mod db;
mod models;
mod handlers;
mod routes;
mod services;
mod schema;
mod config;
mod utils;
mod middleware;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = config::Config::from_env();
    let pool = db::init_db(&config);

    let app = Router::new()
        .nest("/clients", routes::clients::router(pool.clone()))
        .nest("/reservations", routes::reservations::router(pool.clone()))
        .nest("/auth", handlers::auth::router(pool.clone(), config.clone())) // ✅ Adicionando `config`
        .nest("/admin", handlers::admin::router(pool.clone()))
        .layer(middleware::from_fn(auth_middleware)) // ✅ Aplicando autenticação JWT globalmente
        .layer(Extension(pool))
        .layer(Extension(config));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Servidor rodando em http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
