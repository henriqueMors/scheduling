use axum::{Router, Extension, middleware::from_fn};
use std::sync::Arc;
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use tower::ServiceBuilder;

mod db;
mod models;
mod handlers;
mod routes;
mod services;
mod schema;
mod config;
mod utils;
mod middleware;

use crate::routes::professionals;
use crate::middleware::auth_middleware::auth_middleware;
use crate::middleware::rate_limit::{rate_limit_middleware, strict_rate_limit_middleware};
use crate::middleware::cors::cors_middleware;
use crate::handlers::auth::router as auth_router;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // ✅ Inicializa logs com `tracing`
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Falha ao configurar logs");

    // ✅ Carrega configurações
    let config = Arc::new(config::Config::from_env().expect("Failed to load config"));
    let pool = db::init_db(&config);

    tracing::info!("📡 Conectado ao banco de dados");

    // ✅ Rotas abertas (sem autenticação) → RATE LIMIT + CORS
    let auth_routes = auth_router(pool.clone(), config.clone())
        .layer(
            ServiceBuilder::new()
                .layer(rate_limit_middleware())
                .layer(cors_middleware())
        );

    // ✅ Rotas abertas (exemplo com `/health`)
    let open_routes = Router::new()
        .route("/health", axum::routing::get(|| async { "Service is running!" }))
        .layer(cors_middleware());

    // ✅ Rotas protegidas (com autenticação) → RATE LIMIT + CORS + LOGS
    let protected_routes = Router::new()
        .nest("/reservations", routes::reservations::router(pool.clone()))
        .nest("/professionals", professionals::router(pool.clone(), config.clone())) // ✅ Agora incluso corretamente
        .layer(from_fn(auth_middleware))
        .layer(
            ServiceBuilder::new()
                .layer(strict_rate_limit_middleware())
                .layer(cors_middleware())
        );

    // ✅ Aplicação unificada
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
