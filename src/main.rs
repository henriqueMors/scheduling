use axum::{Router, Extension, middleware::from_fn};
use std::sync::Arc;
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod db;
mod models;
mod handlers;
mod routes;
mod services;
mod schema;
mod config;
mod utils;
mod middleware;

use crate::middleware::auth_middleware::auth_middleware;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // 🔹 Inicializa logs estruturados com `tracing`
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Falha ao configurar logs");

    let config = Arc::new(config::Config::from_env());
    let pool = db::init_db(&config);

    tracing::info!("📡 Conectado ao banco de dados");

    // 🔹 Rotas abertas (sem autenticação)
    let auth_routes = handlers::auth::router(pool.clone(), config.clone());

    // 🔹 Rotas protegidas (com autenticação via JWT)
    let protected_routes = Router::new()
        .nest("/clients", routes::clients::router(pool.clone()))
        .nest("/reservations", routes::reservations::router(pool.clone()))
        .nest("/admin", handlers::admin::router(pool.clone()))
        .layer(from_fn(auth_middleware)); // 🔐 Middleware JWT

    let app = Router::new()
        .nest("/auth", auth_routes) // 🔓 Login e registro SEM autenticação
        .merge(protected_routes)    // 🔐 Rotas protegidas COM autenticação
        .layer(Extension(pool))
        .layer(Extension(config));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("🚀 Servidor rodando em http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}