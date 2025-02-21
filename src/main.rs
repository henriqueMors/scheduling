use axum::{Router, Extension};
use std::net::SocketAddr;
use dotenvy::dotenv;
use tokio::net::TcpListener;

mod db;
mod models;
mod handlers; // Certifique-se de que handlers::auth e handlers::admin estejam definidos
mod routes;
mod services;
mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = db::init_db();

    // Agrupa as rotas:
    // - /clients: endpoints do CRUD de clientes
    // - /reservations: endpoints do CRUD de reservas
    // - /auth: endpoints de autenticação (login, verificação e troca de senha)
    // - /admin: endpoints para o administrador master gerenciar administradores secundários
    let app = Router::new()
        .nest("/clients", routes::clients::router(pool.clone()))
        .nest("/reservations", routes::reservations::router(pool.clone()))
        .nest("/auth", handlers::auth::router(pool.clone()))
        .nest("/admin", handlers::admin::admin_router(pool.clone()))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
