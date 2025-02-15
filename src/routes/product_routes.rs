use axum::{routing::{get, post,put}, Router};
use crate::controllers::product_controller;
use std::sync::Arc;
use crate::db::mongo::AppState;

pub fn routes () -> Router<Arc<AppState>>{
    Router::new()
        .route("/products", post(product_controller::create_product))
        .route("/products/finish", post(product_controller::upload_product))
        .route("/products", get(product_controller::get_products))
        .route("/products",put(product_controller::update_product))
        .route("/products/inactive/{id}", post(product_controller::inactive_product_by_id))
        .route("/products/active/{id}", post(product_controller::active_product_by_id))
        .route("/products/delete/{id}", post(product_controller::delete_product))
        .route("/products/{id}", get(product_controller::get_product_with_id))
        .route("/products/full/{id}", get(product_controller::get_product_with_id_and_user))
        .route("/users/{id}/products", get(product_controller::get_user_products))
}