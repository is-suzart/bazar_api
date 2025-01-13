use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::routing::get_service;
use axum::{ routing::get, Router };
use db::mongo::{ create_mongo_client, AppState };
use dotenv::dotenv;
use tower::buffer::BufferLayer;
use tower::limit::RateLimitLayer;
use tower::{ BoxError, ServiceBuilder };
use tracing::Level;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;
use tower_http::services::ServeDir;


mod db;
mod routes;
mod controllers;
mod models;
mod helpers;
mod response;
mod middlewares;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let tracer = tracing_subscriber
        ::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(tracer).unwrap();

    let mongo_client = create_mongo_client().await.unwrap();

    // Inicializa o AppState com o MongoDB
    let app_state = AppState::new(mongo_client);
    let static_route = Router::new().nest_service("/uploads", get_service(ServeDir::new("./uploads")));

    // Usa Arc para gerenciar o estado compartilhado de forma segura
    let shared_state = Arc::new(app_state);

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .merge(routes::user_routes::routes())
        .merge(routes::product_routes::routes())
        .merge(static_route)
        .with_state(shared_state)
        .layer(middlewares::cors_middleware::cors_middleware())
        .layer(
            ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|err: BoxError| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled error: {}", err),
                )
            }))
            .layer(BufferLayer::new(1024))
            .layer(RateLimitLayer::new(5, Duration::from_secs(1))),
    );
        

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
