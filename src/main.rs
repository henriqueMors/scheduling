use axum::Router;
use std::net::SocketAddr;
use dotenvy::dotenv;
use hyper::Server;
use tokio::net::TcpListener;

mod db;
mod models;
mod routes;
mod services;
mod schema; // Expor o schema para todo o crate

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db::init_db();

    let app = Router::new()
        .nest("/clients", routes::clients::router(pool.clone()))
        .nest("/reservations", routes::reservations::router(pool.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    // Cria um TcpListener com Tokio
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    // Passa o listener para o Hyper
    Server::from_tcp(listener)
        .expect("Failed to create server from TCP listener")
        .serve(app.into_make_service())
        .await
        .unwrap();
}
