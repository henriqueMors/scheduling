use axum::{
    extract::{Request, State},
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
pub struct Claims {
    pub sub: String,   // ID do usuário
    pub exp: usize,    // Expiração do token (timestamp UNIX)
    pub role: String,  // Papel do usuário (client, admin, admin_master)
}

/// 🔐 Middleware de autenticação JWT com controle de permissões
pub async fn auth_middleware(
    Extension(config): Extension<Arc<Config>>, // Configuração da aplicação
    mut req: Request<axum::body::Body>,        // Requisição HTTP
    next: Next,                                // Próximo middleware/handler
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
        None => {
            println!("❌ Nenhum token fornecido no cabeçalho.");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // 🔹 Decodifica o JWT
    let key = DecodingKey::from_secret(config.secret_key.as_bytes());
    let decoded = decode::<Claims>(&token, &key, &Validation::default());

    let claims = match decoded {
        Ok(token_data) => token_data.claims,
        Err(e) => {
            println!("❌ Erro ao validar token: {:?}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // 🔹 Verifica a expiração do token
    let now = chrono::Utc::now().timestamp() as usize;
    if claims.exp < now {
        println!("❌ Token expirado!");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // 🔹 Verifica permissões de acesso com base no papel (`role`)
    let path = req.uri().path();

    match claims.role.as_str() {
        "client" => {
            if path.starts_with("/admin") {
                println!("⛔ Acesso negado: Cliente tentando acessar rota de admin.");
                return Err(StatusCode::FORBIDDEN);
            }
        }
        "admin" => {
            if path.starts_with("/admin/add_admin") || path.starts_with("/admin/delete") {
                println!("⛔ Acesso negado: Admin tentando criar/deletar outro admin.");
                return Err(StatusCode::FORBIDDEN);
            }
        }
        "admin_master" => {
            // 🔹 Admin master tem acesso total
        }
        _ => {
            println!("⛔ Acesso negado: Papel desconhecido.");
            return Err(StatusCode::FORBIDDEN);
        }
    }

    // 🔹 Injeta os dados do usuário autenticado na requisição
    req.extensions_mut().insert(claims);

    // 🔹 Passa a requisição adiante
    Ok(next.run(req).await)
}
