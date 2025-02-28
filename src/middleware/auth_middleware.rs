use axum::{
    extract::{FromRequestParts},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::headers::{Authorization, Bearer};
use axum::headers::HeaderMap;
use std::sync::Arc;
use crate::config::Config;
use jsonwebtoken::{decode, DecodingKey, Validation};

/// Middleware para autenticação via JWT.
pub async fn auth_middleware(
    req: Request<axum::body::Body>,
    Extension(config): Extension<Arc<Config>>,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = req.headers();

    // Verifica se existe um cabeçalho Authorization.
    let token = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .map(|t| t.to_string());

    if let Some(token) = token {
        let key = DecodingKey::from_secret(config.secret_key.as_bytes());
        if decode::<serde_json::Value>(&token, &key, &Validation::default()).is_ok() {
            return Ok(next.run(req).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
