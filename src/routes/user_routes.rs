use axum::{routing::{delete, get, post}, Router};
use crate::controllers::user_controller;
use crate::controllers::favorites_controller;
use std::sync::Arc;
use crate::db::mongo::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", post(user_controller::create_user))  // Rota com o handler  // Passando o estado compartilhado
        .route("/users/{id}", get(user_controller::get_user_with_id))
        .route("/login", post(user_controller::login_user))
        .route("/upload-image/{id}" , post(user_controller::upload_profile_picture) )
        .route("/users/favorite", post(favorites_controller::create_favorite)) 
        .route("/users/favorite/{userId}", get(favorites_controller::get_favorites))  
        .route("/users/favorite/{userId}/{id}", delete(favorites_controller::delete_favorite)) 
        
}
