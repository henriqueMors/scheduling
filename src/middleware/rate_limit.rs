use axum::http::{Request, StatusCode};
use std::time::Duration;
use tower_http::limit::RateLimit;
use tower::{ServiceBuilder, ServiceExt};

/// 🔹 Configura Rate Limiting para 5 requisições por segundo por IP
pub fn rate_limit_middleware<S>(service: S) -> impl tower::Service<
    Request<axum::body::Body>,
    Response = S::Response,
    Error = S::Error,
    Future = S::Future
>
where
    S: tower::Service<Request<axum::body::Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    ServiceBuilder::new()
        .layer(RateLimit::new(5, Duration::from_secs(1))) // 🔥 5 req/s por IP
        .service(service)
}

/// 🔹 Configura Rate Limiting mais agressivo para endpoints críticos
pub fn strict_rate_limit_middleware<S>(service: S) -> impl tower::Service<
    Request<axum::body::Body>,
    Response = S::Response,
    Error = S::Error,
    Future = S::Future
>
where
    S: tower::Service<Request<axum::body::Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    ServiceBuilder::new()
        .layer(RateLimit::new(2, Duration::from_secs(1))) // 🔥 2 req/s por IP (mais rigoroso)
        .service(service)
}
