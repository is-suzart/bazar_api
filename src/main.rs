use axum::{ routing::get, Router };
use db::mongo::{ create_mongo_client, AppState };
use dotenv::dotenv;
use tracing::Level;
use std::net::IpAddr;
use std::sync::Arc;
use tracing_subscriber;

mod db;
mod routes;
mod controllers;
mod models;
mod helpers;
mod response;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
    .with_max_level(Level::DEBUG)
    .with_target(true)
    .init();
    dotenv().ok();

    let mongo_client = create_mongo_client().await.unwrap();

    // Inicializa o AppState com o MongoDB
    let app_state = AppState::new(mongo_client);

    // Usa Arc para gerenciar o estado compartilhado de forma segura
    let shared_state = Arc::new(app_state);

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .merge(routes::user_routes::routes())
        .with_state(shared_state);

    println!("🚀 Server started successfully");

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let addr = listener.local_addr().unwrap(); // Get the actual address

    let ip = match addr.ip() {
        IpAddr::V4(ip) => ip.to_string(),
        IpAddr::V6(ip) => format!("[{}]", ip.to_string()), // Enclose IPv6 addresses in brackets
    };
    println!("Server listening on http://{}:{}", ip, addr.port());
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
