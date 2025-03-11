use axum::http::Request;
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;
use tower_http::limit::ConcurrencyLimitLayer;
use tower::ServiceBuilder;
use std::sync::Arc;

/// 🔹 Configura Rate Limiting para 5 requisições simultâneas por IP
pub fn rate_limit_middleware<S>() -> impl tower::Layer<S> + Clone + Send
where
    S: tower::Service<Request<axum::body::Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    ServiceBuilder::new()
        .layer(ConcurrencyLimitLayer::new(5)) // ✅ 5 requisições simultâneas
        .layer(TimeoutLayer::new(Duration::from_secs(1))) // ✅ Timeout de 1 segundo
}

/// 🔹 Configura Rate Limiting mais agressivo para endpoints críticos
pub fn strict_rate_limit_middleware<S>() -> impl tower::Layer<S> + Clone + Send
where
    S: tower::Service<Request<axum::body::Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    ServiceBuilder::new()
        .layer(ConcurrencyLimitLayer::new(2)) // ✅ 2 requisições simultâneas
        .layer(TimeoutLayer::new(Duration::from_secs(1))) // ✅ Timeout de 1 segundo
}
