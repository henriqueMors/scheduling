use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
    Extension,
};
use std::sync::Arc;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::config::Config;
use tracing::{info, error};
use uuid::Uuid;

/// 🔹 Estrutura dos Claims do JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // ID do usuário
    pub exp: usize,  // Expiração do token (timestamp UNIX)
    pub role: String, // Papel do usuário (client, admin, admin_master)
}

/// 🔐 Middleware de autenticação JWT
pub async fn auth_middleware(
    Extension(config): Extension<Arc<Config>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = req.headers();

    // 🔹 Obtém o token do cabeçalho Authorization
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .map(|t| t.to_string());

    let token = match token {
        Some(t) => t,
        None => {
            error!("❌ Nenhum token fornecido no cabeçalho.");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    info!("🔑 Token recebido: {}", token);

    // 🔹 Decodifica o JWT
    let key = DecodingKey::from_secret(config.secret_key.as_bytes());
    let decoded = decode::<Claims>(&token, &key, &Validation::default());

    let claims = match decoded {
        Ok(token_data) => token_data.claims,
        Err(e) => {
            error!("❌ Erro ao validar token: {:?}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // 🔹 Verifica expiração do token
    let now = chrono::Utc::now().timestamp() as usize;
    if claims.exp < now {
        error!("❌ Token expirado!");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // 🔹 Converte o `sub` para `Uuid`
    let user_id = claims
        .sub
        .parse::<Uuid>()
        .map_err(|_| {
            error!("❌ ID inválido no token.");
            StatusCode::BAD_REQUEST
        })?;

    // ✅ Injeta user_id, claims e role na requisição
    req.extensions_mut().insert(user_id); // Uuid
    req.extensions_mut().insert(claims.clone()); // Claims
    req.extensions_mut().insert(claims.role.clone()); // String: role

    info!(
        "✅ Acesso autorizado para usuário com ID: {} (Role: {})",
        user_id, claims.role
    );

    Ok(next.run(req).await)
}

/// 🔒 Middleware para validar papel do usuário
pub async fn require_role(
    required_role: String,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let role = req.extensions().get::<String>().cloned();

    match role {
        Some(user_role) if user_role == required_role || user_role == "admin_master" => {
            info!("✅ Acesso autorizado para role: {}", user_role);
            Ok(next.run(req).await)
        }
        Some(user_role) => {
            error!("❌ Acesso negado para role: {}", user_role);
            Err(StatusCode::FORBIDDEN)
        }
        None => {
            error!("❌ Role não encontrado.");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
