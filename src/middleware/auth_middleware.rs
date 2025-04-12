use axum::{
    extract::Request,
    http::{StatusCode, header, HeaderMap},
    response::Response,
    body::Body,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tracing::{info, error};
use uuid::Uuid;
use std::sync::Arc;
use tower::{Service, Layer};
use chrono::Utc;

/// Claims do token JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,    // ID do usuário
    pub exp: usize,     // Timestamp de expiração
    pub role: String,   // Role do usuário
}

/// Middleware principal de autenticação
#[derive(Clone)]
pub struct AuthMiddleware;

impl<S> Layer<S> for AuthMiddleware {
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
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let (mut parts, body) = req.into_parts();
        let headers = parts.headers.clone();
        let config = match parts.extensions.get::<Arc<crate::config::Config>>() {
            Some(cfg) => Arc::clone(cfg),
            None => {
                error!("Configuração não encontrada nas extensões");
                return Box::pin(async {
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::empty())
                        .unwrap())
                });
            }
        };

        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Extração do token
            let token = headers
                .get(header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer "))
                .map(str::to_owned);

            let token = match token {
                Some(t) => t,
                None => {
                    error!("Nenhum token fornecido no cabeçalho");
                    return Ok(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::empty())
                        .unwrap());
                }
            };

            info!("Token recebido: {}", token);

            // Validação do token
            let key = DecodingKey::from_secret(config.secret_key.as_bytes());
            let decoded = decode::<Claims>(&token, &key, &Validation::default());

            let claims = match decoded {
                Ok(token_data) => token_data.claims,
                Err(e) => {
                    error!("Erro ao validar token: {:?}", e);
                    return Ok(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::empty())
                        .unwrap());
                }
            };

            // Verificação de expiração
            let now = Utc::now().timestamp() as usize;
            if claims.exp < now {
                error!("Token expirado");
                return Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::empty())
                    .unwrap());
            }

            // Validação do ID do usuário
            let user_id = match claims.sub.parse::<Uuid>() {
                Ok(id) => id,
                Err(_) => {
                    error!("ID inválido no token");
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::empty())
                        .unwrap());
                }
            };

            // Adiciona informações ao request
            parts.extensions.insert(user_id);
            parts.extensions.insert(claims.clone());
            parts.extensions.insert(claims.role.clone());
            let req = Request::from_parts(parts, body);

            info!("Acesso autorizado para usuário: {} (Role: {})", user_id, claims.role);

            inner.call(req).await
        })
    }
}

/// Middleware para verificação de roles
#[derive(Clone)]
pub struct RequireRole {
    required_role: String,
}

impl RequireRole {
    pub fn new(required_role: impl Into<String>) -> Self {
        Self {
            required_role: required_role.into(),
        }
    }
}

impl<S> Layer<S> for RequireRole {
    type Service = RequireRoleService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequireRoleService {
            inner,
            required_role: self.required_role.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RequireRoleService<S> {
    inner: S,
    required_role: String,
}

impl<S> Service<Request<Body>> for RequireRoleService<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let role = req.extensions()
            .get::<String>()
            .cloned()
            .unwrap_or_default();
        
        let required_role = self.required_role.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            if role != required_role {
                error!("Acesso negado: Role {} requerida, mas possui {}", required_role, role);
                return Ok(Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body(Body::empty())
                    .unwrap());
            }

            inner.call(req).await
        })
    }
}

// Re-export para facilitar o uso
pub use RequireRole as require_role;