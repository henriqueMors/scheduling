use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use axum::Extension;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::config::Config;

/// 🔹 Estrutura dos Claims do JWT
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,   // ID do usuário
    exp: usize,    // Expiração do token (timestamp UNIX)
    role: String,  // Papel do usuário (client, admin, admin_master)
}

/// 🔐 Middleware de autenticação JWT com controle de permissões
pub async fn auth_middleware(
    Extension(config): Extension<Arc<Config>>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = req.headers();

    // 🔹 Obtém o token do cabeçalho Authorization
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .map(|t| t.to_string());

    // 🔹 Verifica se o token foi fornecido
    let token = match token {
        Some(t) => t,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // 🔹 Decodifica o JWT
    let key = DecodingKey::from_secret(config.secret_key.as_bytes());
    let decoded = decode::<Claims>(&token, &key, &Validation::default());

    let claims = match decoded {
        Ok(token_data) => token_data.claims,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    // 🔹 Verifica a expiração do token
    let now = chrono::Utc::now().timestamp() as usize;
    if claims.exp < now {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // 🔹 Verifica permissões de acesso com base no papel (`role`)
    let path = req.uri().path();

    match claims.role.as_str() {
        "client" => {
            if path.starts_with("/admin") {
                return Err(StatusCode::FORBIDDEN);
            }
        }
        "admin" => {
            if path.starts_with("/admin/add_admin") || path.starts_with("/admin/delete") {
                return Err(StatusCode::FORBIDDEN);
            }
        }
        "admin_master" => {
            // 🔹 Admin master tem acesso total
        }
        _ => return Err(StatusCode::FORBIDDEN),
    }

    // 🔹 Passa a requisição adiante
    Ok(next.run(req).await)
}
