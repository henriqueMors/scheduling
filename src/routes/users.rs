use axum::{
    Router,
    routing::{get, put, delete, patch},
    Extension,
};
use std::sync::Arc;

use crate::{
    db::Pool,
    config::Config,
    middleware::auth_middleware::auth_middleware,
    handlers::user::{
        list_users,
        get_user_by_id,
        update_user,
        delete_user,
        update_user_role,
    },
};

pub fn router(pool: Arc<Pool>, config: Arc<Config>) -> Router {
    Router::new()
        .route("/", get(list_users)) // Rota para listar usuários
        .route("/:id", get(get_user_by_id).put(update_user).delete(delete_user)) // Rota para obter, atualizar ou excluir usuário por ID
        .route("/:id/role", patch(update_user_role)) // Rota para atualizar o papel de um usuário
        .layer(Extension(pool))  // Passando o pool de conexões
        .layer(Extension(config)) // Passando as configurações
        .layer(auth_middleware)  // Aplicando o middleware de autenticação para todas as rotas
}
