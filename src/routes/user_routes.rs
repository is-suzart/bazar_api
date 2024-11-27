use axum::{routing::{get, post}, Router};
use crate::controllers::user_controller;
use std::sync::Arc;
use crate::db::mongo::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", post(user_controller::create_user))  // Rota com o handler  // Passando o estado compartilhado
        .route("/users/:id", get(user_controller::get_user_with_id))
        .route("/login", post(user_controller::login_user))
}
