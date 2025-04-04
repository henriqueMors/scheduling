use axum::{
    extract::Request,
    http::{StatusCode, header},
    response::Response,
    body::Body,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tracing::{info, error};
use uuid::Uuid;
use std::convert::Infallible;
use std::pin::Pin;
use std::future::Future;
use std::task::{Context, Poll};
use tower::Service;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: String,
}

#[derive(Clone)]
pub struct AuthMiddleware;

impl<S> tower::Layer<S> for AuthMiddleware {
    type Service = AuthMiddlewareService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddlewareService { inner }
    }
}

#[derive(Clone)]
pub struct AuthMiddlewareService<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for AuthMiddlewareService<S>
where
    S: Service<Request<Body>, Response = Response, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let config = Arc::clone(req.extensions().get::<Arc<crate::config::Config>>()
            .expect("Config not found in extensions"));

        Box::pin(async move {
            let headers = req.headers();

            let token = headers
                .get(header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer "))
                .map(str::to_owned);

            let token = match token {
                Some(t) => t,
                None => {
                    error!("❌ Nenhum token fornecido no cabeçalho.");
                    return Ok(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::empty())
                        .unwrap());
                }
            };

            info!("🔑 Token recebido: {}", token);

            let key = DecodingKey::from_secret(config.secret_key.as_bytes());
            let decoded = decode::<Claims>(&token, &key, &Validation::default());

            let claims = match decoded {
                Ok(token_data) => token_data.claims,
                Err(e) => {
                    error!("❌ Erro ao validar token: {:?}", e);
                    return Ok(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::empty())
                        .unwrap());
                }
            };

            let now = chrono::Utc::now().timestamp() as usize;
            if claims.exp < now {
                error!("❌ Token expirado!");
                return Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::empty())
                    .unwrap());
            }

            let user_id = match claims.sub.parse::<Uuid>() {
                Ok(id) => id,
                Err(_) => {
                    error!("❌ ID inválido no token.");
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::empty())
                        .unwrap());
                }
            };

            req.extensions_mut().insert(user_id);
            req.extensions_mut().insert(claims.clone());
            req.extensions_mut().insert(claims.role.clone());

            info!("✅ Acesso autorizado para usuário com ID: {} (Role: {})", user_id, claims.role);

            self.inner.call(req).await
        })
    }
}