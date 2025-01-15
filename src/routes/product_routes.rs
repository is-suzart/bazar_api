use axum::{routing::{get, post}, Router};
use crate::controllers::product_controller;
use std::sync::Arc;
use crate::db::mongo::AppState;

pub fn routes () -> Router<Arc<AppState>>{
    Router::new()
        .route("/products", post(product_controller::create_product))
        .route("/products/finish", post(product_controller::upload_product))
        .route("/products", get(product_controller::get_products))
        .route("/products/:id", get(product_controller::get_product_with_id))
        .route("/users/:id/products", get(product_controller::get_user_products))
}