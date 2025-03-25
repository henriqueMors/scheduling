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

    // ✅ Injeta o `user_id`, `role` e `Claims` completos na requisição
    req.extensions_mut().insert(user_id);
    req.extensions_mut().insert(claims.clone()); // Aqui inserimos os Claims completos
    req.extensions_mut().insert(claims.role.clone()); // E o role também, caso necessário

    info!(
        "✅ Acesso autorizado para usuário com ID: {} (Role: {})",
        user_id, claims.role
    );

    // 🔹 Passa a requisição adiante
    Ok(next.run(req).await)
}
