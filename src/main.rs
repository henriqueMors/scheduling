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
use crate::handlers::auth::router as auth_router;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // 🔹 Inicializa logs estruturados com `tracing`
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Falha ao configurar logs");

    // 🔹 Carrega as configurações
    let config = Arc::new(config::Config::from_env().map_err(|e| panic!("Config error: {}", e)).unwrap());
    let pool = db::init_db(&config);

    tracing::info!("📡 Conectado ao banco de dados");

    // 🔹 Rotas abertas (sem autenticação)
    let auth_routes = auth_router(pool.clone(), config.clone());

    let open_routes = Router::new();

    // 🔹 Rotas protegidas (com autenticação via JWT)
    let protected_routes = Router::new()
        .nest("/reservations", routes::reservations::router(pool.clone()))
        // ❌ Removido `admin_router`
        .layer(from_fn(auth_middleware));

    let app = Router::new()
        .nest("/auth", auth_routes)
        .merge(open_routes)
        .merge(protected_routes)
        .layer(Extension(pool))
        .layer(Extension(config));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("🚀 Servidor rodando em http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
