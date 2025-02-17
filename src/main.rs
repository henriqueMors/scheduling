use axum::Router;
use std::net::SocketAddr;
use dotenvy::dotenv;
use hyper::server::Server; // Importa Server diretamente do hyper

mod db;
mod models;
mod routes;
mod services;
mod schema; // Para expor o schema para todo o crate

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db::init_db();

    let app = Router::new()
        .nest("/clients", routes::clients::router(pool.clone()))
        .nest("/reservations", routes::reservations::router(pool.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
