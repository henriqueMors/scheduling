use axum::{Router, Extension, middleware::from_fn, routing::post};
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
use crate::routes::clients::{router as clients_router, create_client};

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
    let auth_routes = auth_router(pool.clone(), config.clone());

    let open_routes = Router::new()
        .route("/clients", post(create_client)); // 🔓 Criar cliente sem autenticação

    // 🔹 Rotas protegidas (com autenticação via JWT)
    let protected_routes = Router::new()
        .nest("/clients", clients_router(pool.clone())) // 🔐 Protege as demais rotas de clients
        .nest("/reservations", routes::reservations::router(pool.clone()))
        .nest("/admin", handlers::admin::router(pool.clone()))
        .layer(from_fn(auth_middleware)); // 🔐 Middleware JWT

    let app = Router::new()
        .nest("/auth", auth_routes)  // 🔓 Login e registro SEM autenticação
        .merge(open_routes)          // 🔓 Criar cliente SEM autenticação
        .merge(protected_routes)     // 🔐 Restante das rotas protegidas
        .layer(Extension(pool))
        .layer(Extension(config));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("🚀 Servidor rodando em http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
