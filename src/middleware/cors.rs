use tower_http::cors::{CorsLayer, Any};
use std::time::Duration;

/// 🔐 Configuração de CORS para permitir apenas origens confiáveis
pub fn cors_middleware() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("http://localhost:3000".parse().unwrap()) // 🔥 Permite apenas o front-end local
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::PUT, axum::http::Method::DELETE])
        .allow_headers([axum::http::header::AUTHORIZATION, axum::http::header::CONTENT_TYPE])
        .max_age(Duration::from_secs(600)) // 🔥 Cache de 10 minutos para o preflight
}
