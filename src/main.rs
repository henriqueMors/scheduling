use axum::{Router, routing::get};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use dotenvy::dotenv;
use tower::Service; 
use tower::ServiceExt;
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

    let app = Arc::new(
        Router::new()
            .nest("/clients", routes::clients::router(pool.clone()))
            .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("🚀 Server running on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let app = app.clone();

        tokio::spawn(async move {
            let io = TokioIo::new(stream);

            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(move |req| {
                    let app = app.clone();
                    async move { app.clone().oneshot(req).await }
                }))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
