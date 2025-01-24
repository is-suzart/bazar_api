use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use crate::{db::{favorites_db::get_favorite_products, mongo::AppState}, models::product_models::{FavoriteProduct, Product}};
use std::sync::Arc;

#[tracing::instrument]
pub async fn get_favorites(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>
) -> impl IntoResponse {
    match get_favorite_products(&state, &id).await {
       Ok(favorites) if !favorites.is_empty() => {
        let parsed_products: Vec<Product> = favorites
        .into_iter()
        .filter_map(|doc| bson::from_bson(bson::Bson::Document(doc)).ok())
        .collect();
    (StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "message" : "Produtos recebidos com sucesso",
            "products": parsed_products
        })) )
        
    }
    Ok(_) => {
        (StatusCode::NOT_FOUND,
        Json(serde_json::json!({
            "status": "error",
            "message": "Produto não encontrado"
        })))
    }
    Err(err) => (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({
            "status": "error",
            "message": format!("Erro ao atualizar o produto: {}", err),
        })),
    )
        
    }
}

#[tracing::instrument]
pub async fn create_favorite(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<FavoriteProduct>
) -> impl IntoResponse {
    match crate::db::favorites_db::post_favorite(&state, &payload).await {
        Ok(_) => {
            (StatusCode::CREATED, Json(serde_json::json!({
                "status": "success",
                "message": "Produto favoritado com sucesso"
            })))
        }
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "status": "error",
                "message": format!("Erro ao favoritar o produto: {}", err)
            })))
        }
    }
}

pub async fn delete_favorite(
    State(state): State<Arc<AppState>>,
    Path((user_id, product_id)): Path<(String, String)>
) -> impl IntoResponse {
    match crate::db::favorites_db::delete_favorite(&state, &user_id, &product_id).await {
        Ok(_) => {
            (StatusCode::OK, Json(serde_json::json!({
                "status": "success",
                "message": "Produto desfavoritado com sucesso"
            })))
        }
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "status": "error",
                "message": format!("Erro ao desfavoritar o produto: {}", err)
            })))
        }
    }
}