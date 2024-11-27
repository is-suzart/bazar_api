pub mod user_routes;

use std::sync::Arc;

use axum::Router;

use crate::db::mongo::AppState;

#[allow(dead_code)]
pub fn user_routes() -> Router<Arc<AppState>> {
    user_routes::routes()
}
