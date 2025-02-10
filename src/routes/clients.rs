use axum::{routing::get, Router, Json, extract::State};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::db::DbPool;
use crate::models::client::Client;

#[derive(Debug, Deserialize, Serialize)]
struct NewClient {
    name: String,
    phone: String,
    email: String,
}

pub fn router(pool: DbPool) -> Router {
    Router::new()
        .route("/", get(list_clients))
        .route("/", axum::routing::post(create_client))
        .with_state(pool)
}

async fn list_clients(State(pool): State<DbPool>) -> Json<Vec<Client>> {
    use crate::schema::clients::dsl::*;

    let mut conn = pool.get().expect("Failed to get DB connection");
    let results = clients
        .load::<Client>(&mut conn)
        .expect("Error loading clients");

    Json(results)
}

async fn create_client(
    State(pool): State<DbPool>,
    Json(new_client): Json<NewClient>,
) -> Json<Client> {
    use crate::schema::clients::dsl::*;

    let mut conn = pool.get().expect("Failed to get DB connection");

    let client = Client {
        id: uuid::Uuid::new_v4(),
        name: new_client.name,
        phone: new_client.phone,
        email: Some(new_client.email),
    };

    diesel::insert_into(clients)
        .values(&client)
        .execute(&mut conn)
        .expect("Failed to insert client");

    Json(client)
}