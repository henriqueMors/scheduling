use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};
use headers::{Authorization, HeaderMapExt};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::sync::Arc;
use crate::config::Config;
use crate::models::user::User;
use crate::db::Pool;
use crate::schema::users::dsl::*;
use diesel::prelude::*;

/// 🔹 Estrutura dos Claims do JWT
#[derive(Debug, serde::Deserialize)]
struct Claims {
    sub: String,  // ID do usuário
    exp: usize,   // Expiração do token
    role: String, // Papel do usuário
}

/// 🔐 Middleware de autenticação e controle de permissões
pub async fn auth_middleware<B>(
    State(pool): State<Pool>,
    Extension(config): Extension<Arc<Config>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let headers = req.headers();

    // 🔹 Obtém o token do header Authorization
    let token = headers
        .typed_get::<Authorization<String>>()
        .map(|auth| auth.0)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 🔹 Decodifica o JWT
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(config.secret_key.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let claims = decoded.claims;

    // 🔹 Busca o usuário no banco de dados para verificar se ainda existe
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let user = users
        .filter(id.eq(&claims.sub))
        .first::<User>(&mut conn)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // 🔹 Verifica permissões específicas com base no papel do usuário (`role`)
    let path = req.uri().path();

    match user.role.as_str() {
        "client" => {
            if path.starts_with("/admin") || path.starts_with("/clients") && path != format!("/clients/{}", user.id) {
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

    // 🔹 Injeta o usuário autenticado na requisição (se necessário)
    req.extensions_mut().insert(user);

    // 🔹 Passa a requisição adiante
    Ok(next.run(req).await)
}
