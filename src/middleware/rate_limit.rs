use axum::http::Request;
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;
use tower_http::add_extension::AddExtensionLayer;
use tower::{ServiceBuilder, Service};
use std::sync::Arc;
use tokio::sync::Semaphore;

/// 🔹 Configura Rate Limiting para 5 requisições simultâneas por IP
pub fn rate_limit_middleware<S>() -> impl tower::Layer<S> + Clone + Send
where
    S: Service<Request<axum::body::Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    let semaphore = Arc::new(Semaphore::new(5)); // ✅ 5 requisições simultâneas

    ServiceBuilder::new()
        .layer(AddExtensionLayer::new(semaphore.clone()))
        .layer(TimeoutLayer::new(Duration::from_secs(1))) // ✅ Timeout de 1 segundo
}

/// 🔹 Configura Rate Limiting mais agressivo para endpoints críticos
pub fn strict_rate_limit_middleware<S>() -> impl tower::Layer<S> + Clone + Send
where
    S: Service<Request<axum::body::Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    let semaphore = Arc::new(Semaphore::new(2)); // ✅ 2 requisições simultâneas

    ServiceBuilder::new()
        .layer(AddExtensionLayer::new(semaphore.clone()))
        .layer(TimeoutLayer::new(Duration::from_secs(1))) // ✅ Timeout de 1 segundo
}
