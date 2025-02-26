use axum::{Router, Extension};
use std::net::SocketAddr;
use dotenvy::dotenv;
use tokio::net::TcpListener;

mod db;
mod models;
mod handlers; // Certifique-se de que handlers::auth, handlers::admin e handlers::calendar estejam definidos
mod routes;
mod services;
mod schema;

#[tokio::main]
async fn main() {
    // Carrega as variáveis de ambiente do arquivo .env
    dotenv().ok();

    // Inicializa o pool de conexões com o banco de dados
    let pool = db::init_db();

    // Agrupa as rotas:
    // - /clients: endpoints do CRUD de clientes
    // - /reservations: endpoints do CRUD de reservas
    // - /auth: endpoints de autenticação (login, verificação, troca e recuperação de senha)
    // - /admin: endpoints para o administrador master gerenciar administradores secundários
    // - /calendar: endpoint para exibir o calendário de agendamentos
    let app = Router::new()
        .nest("/clients", routes::clients::router(pool.clone()))
        .nest("/reservations", routes::reservations::router(pool.clone()))
        .nest("/auth", handlers::auth::router(pool.clone()))
        .nest("/admin", handlers::admin::admin_router(pool.clone()))
        .nest("/calendar", handlers::calendar::router(pool.clone()))
        .layer(Extension(pool));

    // Define o endereço e porta para o servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    // Cria um TcpListener e inicia o servidor Axum
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
