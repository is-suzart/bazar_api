use axum::{extract::{Json, Path, State,Multipart}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use crate::{db::{mongo::AppState, user_db::{insert_user, query_user_by_email, query_user_by_id}}, helpers::{password::verify, token::generate_jwt}, models::user_models::{LoginUserModel, ResponseUser}, response::user_response::{CreateUserResponse, QueryUserResponse}};
use crate:: models::user_models::{CreateUserModel, User};
use std::sync::Arc;
use crate::helpers::password;
use uuid::Uuid;
use std::{fs, path::Path as StdPath};

pub async fn create_favorite(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>
) -> impl IntoResponse {
    match get_favorite_products(state.favorites_collection.clone(), &id).await {
        Ok(products) => {
            let response = json!({ "products": products });
            (StatusCode::OK, response)
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, json!({ "message": "Internal server error" }))
    }
}