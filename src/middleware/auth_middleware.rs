use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use axum::Extension;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::config::Config;

/// Middleware de autenticação JWT.
pub async fn auth_middleware(
    Extension(config): Extension<Arc<Config>>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = req.headers();

    // Obtém o token do cabeçalho Authorization
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .map(|t| t.to_string());

    if let Some(token) = token {
        let key = DecodingKey::from_secret(config.secret_key.as_bytes());
        if decode::<serde_json::Value>(&token, &key, &Validation::default()).is_ok() {
            return Ok(next.run(req).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
