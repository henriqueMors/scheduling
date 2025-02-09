use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/", post(criar_cliente))
        .route("/", get(listar_clientes))
        .with_state(pool)
}

#[derive(Debug, Serialize, Deserialize)]
struct NovoCliente {
    nome: String,
    telefone: String,
    email: Option<String>,
}

async fn criar_cliente(
    State(pool): State<PgPool>,
    Json(novo_cliente): Json<NovoCliente>,
) -> Result<Json<String>, String> {
    let id = Uuid::new_v4();
    let res = sqlx::query!(
        "INSERT INTO clientes (id, nome, telefone, email) VALUES ($1, $2, $3, $4)",
        id,
        novo_cliente.nome,
        novo_cliente.telefone,
        novo_cliente.email
    )
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok(Json(format!("Cliente {} criado com sucesso!", id))),
        Err(e) => Err(format!("Erro ao criar cliente: {}", e)),
    }
}

async fn listar_clientes(State(pool): State<PgPool>) -> Result<Json<Vec<NovoCliente>>, String> {
    let clientes = sqlx::query_as!(
        NovoCliente,
        "SELECT nome, telefone, email FROM clientes"
    )
    .fetch_all(&pool)
    .await;

    match clientes {
        Ok(lista) => Ok(Json(lista)),
        Err(e) => Err(format!("Erro ao listar clientes: {}", e)),
    }
}
