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
use crate::routes::reservations::router as reservations_router;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // 🔹 Inicializa logs estruturados com `tracing`
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("❌ Falha ao configurar logs");

    // 🔹 Inicializa o banco de dados
    let config = Arc::new(config::Config::from_env());
    let pool = db::init_db(&config).expect("❌ Falha ao conectar ao banco de dados");

    tracing::info!("📡 Conectado ao banco de dados");

    // 🔓 Rotas abertas (sem autenticação)
    let auth_routes = auth_router(pool.clone(), config.clone());

    // 🔐 Rotas protegidas (com autenticação via JWT)
    let protected_routes = Router::new()
        .nest("/clients", routes::clients::router(pool.clone())) // ✅ Usa o router correto
        .nest("/reservations", reservations_router(pool.clone()))
        .nest("/admin", admin_router(pool.clone()))
        .layer(from_fn(auth_middleware)); // ✅ Aplica middleware de autenticação

    // 🔹 Define o app combinando rotas abertas e protegidas
    let app = Router::new()
        .nest("/auth", auth_routes)  // 🔓 Login e registro SEM autenticação
        .merge(protected_routes)     // 🔐 Rotas protegidas com JWT
        .layer(Extension(pool))
        .layer(Extension(config));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("🚀 Servidor rodando em http://{}", addr);

    // 🔥 Inicia o servidor
    let listener = TcpListener::bind(addr)
        .await
        .expect("❌ Falha ao vincular o endereço");

    axum::serve(listener, app.into_make_service())
        .await
        .expect("❌ Falha ao iniciar o servidor");
}
