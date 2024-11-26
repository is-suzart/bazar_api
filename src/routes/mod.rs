pub mod user_routes;

use std::sync::Arc;

use axum::Router;

use crate::db::mongo::AppState;

// Função que combina todas as rotas
pub fn user_routes() -> Router<Arc<AppState>> {
    user_routes::routes()
}
