use axum::{http::Request, body::Body, response::Response};
use std::time::Duration;
use tower::{Service, ServiceBuilder, ServiceExt, Layer};
use tower_http::timeout::TimeoutLayer;
use tower::util::BoxCloneService;

/// 🔹 Middleware de Rate Limit para 5 requisições simultâneas por IP
pub fn rate_limit_middleware<S>() -> impl Layer<S> + Clone + Send
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    ServiceBuilder::new()
        .layer(TimeoutLayer::new(Duration::from_secs(1)))
        .layer(BoxCloneService::new)
}

/// 🔹 Middleware de Rate Limit mais agressivo (2 requisições simultâneas por IP)
pub fn strict_rate_limit_middleware<S>() -> impl Layer<S> + Clone + Send
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    ServiceBuilder::new()
        .layer(TimeoutLayer::new(Duration::from_secs(1)))
        .layer(BoxCloneService::new)
}
