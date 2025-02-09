use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{db::DbPool, models::client::Client, schema::clients};

pub fn router(pool: DbPool) -> Router {
    Router::new()
        .route("/", post(create_client))
        .route("/", get(get_clients))
        .with_state(pool)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct NewClient {
    name: String,
    phone: String,
    email: Option<String>,
}

#[axum::debug_handler]
async fn create_client(
    State(pool): State<DbPool>,
    Json(new_client): Json<NewClient>,
) -> Result<Json<String>, String> {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let client = Client {
        id: Uuid::new_v4(),
        name: new_client.name,
        phone: new_client.phone,
        email: new_client.email,
    };

    diesel::insert_into(clients::table)
        .values(&client)
        .execute(conn)
        .expect("Error inserting client");

    Ok(Json(format!("Client {} created!", client.id)))
}

#[axum::debug_handler]
async fn get_clients(State(pool): State<DbPool>) -> Result<Json<Vec<Client>>, String> {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let result = clients::table
        .load::<Client>(conn)
        .expect("Error loading clients");

    Ok(Json(result))
}
