use axum::http::Request;
use std::time::Duration;
use tower_http::limit::RateLimitLayer;
use tower::ServiceBuilder;

/// 🔹 Configura Rate Limiting para 5 requisições por segundo por IP
pub fn rate_limit_middleware<S>() -> ServiceBuilder<RateLimitLayer>
where
    S: tower::Service<Request<axum::body::Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    ServiceBuilder::new()
        .layer(RateLimitLayer::new(5, Duration::from_secs(1))) // ✅ 5 req/s por IP
}

/// 🔹 Configura Rate Limiting mais agressivo para endpoints críticos
pub fn strict_rate_limit_middleware<S>() -> ServiceBuilder<RateLimitLayer>
where
    S: tower::Service<Request<axum::body::Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    ServiceBuilder::new()
        .layer(RateLimitLayer::new(2, Duration::from_secs(1))) // ✅ 2 req/s por IP (mais rigoroso)
}
