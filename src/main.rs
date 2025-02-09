use axum::{Router, routing::get};
use hyper::Server;
use std::net::SocketAddr;
use dotenvy::dotenv;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod db;
mod routes;
mod models;
mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = db::init_db();

    let app = Router::new()
        .nest("/clients", routes::clients::router(pool.clone()))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Server running on http://{}", addr);

    Server::bind(&addr)
    .serve(app.into_make_service())
        .await
        .unwrap();
}
