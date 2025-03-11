use axum::http::Request;
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;
use tower::{ServiceBuilder, Layer, BoxCloneService};
use hyper::Body;

/// 🔹 Configura Rate Limiting para 5 requisições simultâneas por IP
pub fn rate_limit_middleware<S>() -> BoxCloneService<Request<Body>, S::Response, S::Error>
where
    S: tower::Service<Request<Body>> + Clone + Send + 'static,
    S::Response: Send + 'static,
    S::Future: Send + 'static,
{
    ServiceBuilder::new()
        .layer(TimeoutLayer::new(Duration::from_secs(1)))
        .service_fn(|req, svc| async move { svc.call(req).await })
        .boxed_clone()
}

/// 🔹 Configura Rate Limiting mais agressivo para endpoints críticos
pub fn strict_rate_limit_middleware<S>() -> BoxCloneService<Request<Body>, S::Response, S::Error>
where
    S: tower::Service<Request<Body>> + Clone + Send + 'static,
    S::Response: Send + 'static,
    S::Future: Send + 'static,
{
    ServiceBuilder::new()
        .layer(TimeoutLayer::new(Duration::from_secs(1)))
        .service_fn(|req, svc| async move { svc.call(req).await })
        .boxed_clone()
}
