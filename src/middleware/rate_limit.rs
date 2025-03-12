use axum::http::{Request, Response};
use axum::body::Body;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::Semaphore;
use tower::{Layer, Service, ServiceBuilder};
use tower_http::timeout::TimeoutLayer;
use std::time::Duration;
use std::future::Future;
use std::pin::Pin;

#[derive(Clone)]
pub struct RateLimitMiddleware<S> {
    inner: S,
    semaphore: Arc<Semaphore>,
}

impl<S> Service<Request<Body>> for RateLimitMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let semaphore = self.semaphore.clone();
        let permit = semaphore.try_acquire_owned();

        match permit {
            Ok(_permit) => {
                let future = self.inner.call(req);
                Box::pin(async move { future.await })
            }
            Err(_) => Box::pin(async {
                let response = Response::builder()
                    .status(429) // HTTP 429: Too Many Requests
                    .body(Body::from("Too many requests"))
                    .unwrap();
                Ok(response)
            }),
        }
    }
}

#[derive(Clone)]
pub struct RateLimitLayer {
    semaphore: Arc<Semaphore>,
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitMiddleware {
            inner,
            semaphore: self.semaphore.clone(),
        }
    }
}

/// 🔹 Configura Rate Limiting para 5 requisições simultâneas por IP
pub fn rate_limit_middleware<S>() -> impl Layer<S> + Clone + Send
where
    S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    ServiceBuilder::new()
        .layer(TimeoutLayer::new(Duration::from_secs(1)))
        .layer(RateLimitLayer {
            semaphore: Arc::new(Semaphore::new(5)),
        })
}

/// 🔹 Configura Rate Limiting mais agressivo para endpoints críticos
pub fn strict_rate_limit_middleware<S>() -> impl Layer<S> + Clone + Send
where
    S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    ServiceBuilder::new()
        .layer(TimeoutLayer::new(Duration::from_secs(1)))
        .layer(RateLimitLayer {
            semaphore: Arc::new(Semaphore::new(2)),
        })
}
