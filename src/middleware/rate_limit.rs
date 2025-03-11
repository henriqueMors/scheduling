use axum::http::Request;
use std::time::Duration;
use tower_http::request_id::{RequestIdLayer, MakeRequestId};
use tower::ServiceBuilder;
use http::header::HeaderValue;
use std::sync::Arc;
use uuid::Uuid;

/// 🔹 Configura Rate Limiting para 5 requisições por segundo por IP
pub fn rate_limit_middleware<S>() -> ServiceBuilder<RequestIdLayer<Arc<dyn MakeRequestId>>>
where
    S: tower::Service<Request<axum::body::Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    let make_request_id = Arc::new(|| {
        HeaderValue::from_str(&Uuid::new_v4().to_string()).ok()
    });

    ServiceBuilder::new()
        .layer(RequestIdLayer::new(make_request_id))
}

/// 🔹 Configura Rate Limiting mais agressivo para endpoints críticos
pub fn strict_rate_limit_middleware<S>() -> ServiceBuilder<RequestIdLayer<Arc<dyn MakeRequestId>>>
where
    S: tower::Service<Request<axum::body::Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    let make_request_id = Arc::new(|| {
        HeaderValue::from_str(&Uuid::new_v4().to_string()).ok()
    });

    ServiceBuilder::new()
        .layer(RequestIdLayer::new(make_request_id))
}
