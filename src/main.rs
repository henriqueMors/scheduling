use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use dotenv::dotenv;

mod config;
mod db;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Carregar variáveis de ambiente
    let pool = db::init_db().await.expect("Falha ao conectar no banco");

    let app = Router::new()
        .nest("/clientes", routes::clientes::router(pool.clone()))
        .nest("/reservas", routes::reservas::router(pool.clone()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Servidor rodando em http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
