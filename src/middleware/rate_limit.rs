use axum::http::Request;
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;
use tower_http::limit::ConcurrencyLimitLayer;
use std::time::Duration;

/// 🔹 Configura Rate Limiting para 5 requisições simultâneas por IP
pub fn rate_limit_middleware<S>() -> ServiceBuilder<impl tower::Layer<S>>
where
    S: tower::Service<Request<axum::body::Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    ServiceBuilder::new()
        .layer(ConcurrencyLimitLayer::new(5)) // ✅ Limite de 5 requisições simultâneas
        .layer(TimeoutLayer::new(Duration::from_secs(1))) // ✅ Timeout de 1 segundo para cada requisição
}

/// 🔹 Configura Rate Limiting mais agressivo para endpoints críticos
pub fn strict_rate_limit_middleware<S>() -> ServiceBuilder<impl tower::Layer<S>>
where
    S: tower::Service<Request<axum::body::Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    ServiceBuilder::new()
        .layer(ConcurrencyLimitLayer::new(2)) // ✅ Limite de 2 requisições simultâneas (mais rígido)
        .layer(TimeoutLayer::new(Duration::from_secs(1))) // ✅ Timeout de 1 segundo para cada requisição
}
