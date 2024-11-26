use axum::{routing::post, Extension, Router};
use crate::controllers::user_controller;
use std::sync::Arc;
use crate::db::mongo::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", post(user_controller::create_user))  // Rota com o handler  // Passando o estado compartilhado
}
