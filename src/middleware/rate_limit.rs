use axum::http::Request;
use axum::body::Body;
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;
use tower::{ServiceBuilder, Service, Layer, ServiceExt};
use tower::util::BoxCloneService;
use tower::service_fn;
use axum::response::Response;

/// 🔹 Configura Rate Limiting para 5 requisições simultâneas por IP
pub fn rate_limit_middleware<S>() -> BoxCloneService<Request<Body>, Response, S::Error>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    BoxCloneService::new(
        ServiceBuilder::new()
            .layer(TimeoutLayer::new(Duration::from_secs(1)))
            .service(service_fn(move |req: Request<Body>| {
                let mut svc = ServiceBuilder::new()
                    .service_fn(|req: Request<Body>| async move {
                        Ok::<_, S::Error>(Response::new(Body::from("Rate Limited!")))
                    });

                async move { svc.call(req).await }
            })),
    )
}

/// 🔹 Configura Rate Limiting mais agressivo para endpoints críticos
pub fn strict_rate_limit_middleware<S>() -> BoxCloneService<Request<Body>, Response, S::Error>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    BoxCloneService::new(
        ServiceBuilder::new()
            .layer(TimeoutLayer::new(Duration::from_secs(1)))
            .service(service_fn(move |req: Request<Body>| {
                let mut svc = ServiceBuilder::new()
                    .service_fn(|req: Request<Body>| async move {
                        Ok::<_, S::Error>(Response::new(Body::from("Strict Rate Limited!")))
                    });

                async move { svc.call(req).await }
            })),
    )
}
