use axum::{
    extract::RequestParts,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};
use headers::{HeaderMapExt, Authorization};
use crate::config::Config;
use crate::services::auth_service::validate_jwt;
use std::sync::Arc;

pub async fn auth_middleware<B>(
    mut req: Request<B>,
    Extension(config): Extension<Arc<Config>>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let headers = req.headers().clone();

    // 🔹 Obtém o token JWT do cabeçalho Authorization
    let token = headers
        .typed_get::<Authorization<String>>()
        .map(|auth| auth.0);

    if let Some(token) = token {
        if let Ok(user_id) = validate_jwt(&token, &config) {
            req.extensions_mut().insert(user_id);
            return Ok(next.run(req).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub fn auth_middleware() {
    println!("Middleware de autenticação funcionando!");
}
