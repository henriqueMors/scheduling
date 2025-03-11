use axum::http::{Request, Response};
use std::time::Duration;
use tower::{Service, ServiceBuilder, Layer};
use tower_http::{timeout::TimeoutLayer, limit::ConcurrencyLimitLayer};

/// 🔹 Middleware de Rate Limit para 5 requisições simultâneas por IP
pub fn rate_limit_middleware<S>() -> impl Layer<S> + Clone + Send
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    ServiceBuilder::new()
        .layer(ConcurrencyLimitLayer::new(5)) // 🔥 Limite de 5 conexões simultâneas
        .layer(TimeoutLayer::new(Duration::from_secs(1))) // 🔥 Timeout de 1s
}

/// 🔹 Middleware de Rate Limit mais agressivo (2 requisições simultâneas por IP)
pub fn strict_rate_limit_middleware<S>() -> impl Layer<S> + Clone + Send
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    ServiceBuilder::new()
        .layer(ConcurrencyLimitLayer::new(2)) // 🔥 Apenas 2 conexões simultâneas
        .layer(TimeoutLayer::new(Duration::from_secs(1))) // 🔥 Timeout de 1s
}
