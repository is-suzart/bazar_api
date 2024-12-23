use axum::{routing::{get, post}, Router};
use crate::controllers::product_controller;
use std::sync::Arc;
use crate::db::mongo::AppState;

pub fn routes () -> Router<Arc<AppState>>{
    Router::new()
        .route("/products", post(product_controller::create_product))
        .route("/products/finish", post(product_controller::upload_product))
        .route("/products", get(root))
}

async fn root() -> &'static str {
    "Hello, World!"
}