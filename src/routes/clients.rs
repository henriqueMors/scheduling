use axum::{routing::{get, post, put, delete}, Router, Json, extract::{Path, State}};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{db::DbPool, models::client::{Client, NewClient}, schema::clients::dsl::*};

pub fn router(pool: DbPool) -> Router {
    Router::new()
        .route("/", get(list_clients).post(create_client))
        .route("/:id", get(get_client).put(update_client).delete(delete_client))
        .with_state(pool)
}

// 🔹 Listar todos os clientes (GET /clients)
async fn list_clients(State(pool): State<DbPool>) -> Json<Vec<Client>> {
    use crate::schema::clients::dsl::*;
    let mut conn = pool.get().expect("Falha ao obter conexão do banco");
    let results = clients.load::<Client>(&mut conn).expect("Erro ao buscar clientes");
    Json(results)
}

// 🔹 Criar novo cliente (POST /clients)
#[derive(Deserialize)]
struct CreateClient {
    name: String,
    phone: String,
    email: Option<String>,
}

async fn create_client(State(pool): State<DbPool>, Json(payload): Json<CreateClient>) -> Json<Client> {
    let mut conn = pool.get().expect("Falha ao obter conexão do banco");
    
    let new_client = NewClient {
        id: Uuid::new_v4(),
        name: payload.name,
        phone: payload.phone,
        email: payload.email,
    };

    diesel::insert_into(clients)
        .values(&new_client)
        .execute(&mut conn)
        .expect("Erro ao inserir cliente");

    Json(Client {
        id: new_client.id,
        name: new_client.name,
        phone: new_client.phone,
        email: new_client.email,
    })
}

// 🔹 Buscar cliente pelo ID (GET /clients/:id)
async fn get_client(State(pool): State<DbPool>, Path(client_id): Path<Uuid>) -> Json<Client> {
    let mut conn = pool.get().expect("Falha ao obter conexão do banco");
    let client = clients
        .filter(id.eq(client_id))
        .first::<Client>(&mut conn)
        .expect("Cliente não encontrado");

    Json(client)
}

// 🔹 Atualizar cliente (PUT /clients/:id)
#[derive(Deserialize)]
struct UpdateClient {
    name: Option<String>,
    phone: Option<String>,
    email: Option<String>,
}

async fn update_client(State(pool): State<DbPool>, Path(client_id): Path<Uuid>, Json(payload): Json<UpdateClient>) -> Json<Client> {
    let mut conn = pool.get().expect("Falha ao obter conexão do banco");

    diesel::update(clients.filter(id.eq(client_id)))
        .set((
            name.eq(payload.name.unwrap_or_else(|| "".to_string())),
            phone.eq(payload.phone.unwrap_or_else(|| "".to_string())),
            email.eq(payload.email),
        ))
        .execute(&mut conn)
        .expect("Erro ao atualizar cliente");

    let updated_client = clients
        .filter(id.eq(client_id))
        .first::<Client>(&mut conn)
        .expect("Cliente não encontrado");

    Json(updated_client)
}

// 🔹 Deletar cliente (DELETE /clients/:id)
async fn delete_client(State(pool): State<DbPool>, Path(client_id): Path<Uuid>) -> Json<String> {
    let mut conn = pool.get().expect("Falha ao obter conexão do banco");

    diesel::delete(clients.filter(id.eq(client_id)))
        .execute(&mut conn)
        .expect("Erro ao deletar cliente");

    Json(format!("Cliente {} removido!", client_id))
}
