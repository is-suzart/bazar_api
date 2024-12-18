use axum::{extract::{Json, State}, http::StatusCode, response::IntoResponse};
use tracing::{info, error, debug};
use std::sync::Arc;
use crate::{db::mongo::AppState, models::product_models::Storage};
use crate::models::product_models::{CreateProductModel, Product};
use crate::db::product_db::insert_product;


#[tracing::instrument]
pub async fn create_product(
    State(state): State<Arc<AppState>>, Json(payload): Json<CreateProductModel>,  // Recebe o payload da requisição
) -> impl IntoResponse {
    let storage: Storage = Storage::new(payload.storage);
    let product = Product::new(
        payload.user_id,
        payload.info,
        storage
    );
    match insert_product(&state, &product).await {
        Ok(_insert_result) => {
            info!("Produto criado com sucesso: {}", &product.id);
            (
            
                StatusCode::CREATED,
                Json(serde_json::json!({ "status": "success", "product_id": &product.id, "message": "Produto criado com sucesso!" }))
            )
        } ,
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "status": "error", "message": err.to_string() }))
        )
    }

}