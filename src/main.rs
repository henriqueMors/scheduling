use axum::Router;
use std::net::SocketAddr;
use dotenvy::dotenv;
use tokio::net::TcpListener;

mod db;
mod models;
mod handlers; // Certifique-se de que o módulo handlers inclua auth
mod routes;
mod services;
mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db::init_db();

    let app = Router::new()
        .nest("/clients", routes::clients::router(pool.clone()))
        .nest("/reservations", routes::reservations::router(pool.clone()))
        .nest("/auth/admin", handlers::auth::router(pool.clone())) // Rota de autenticação para administradores
        .nest("/auth/client", handlers::auth::router(pool.clone())); // Alterei para `router` (não `client_router`)

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
