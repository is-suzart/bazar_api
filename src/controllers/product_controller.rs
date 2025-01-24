use axum::{extract::{Json, Multipart, Path, Query, State}, http::StatusCode, response::IntoResponse};
use tracing::info;
use std::{fs, sync::Arc};
use crate::{db::{mongo::AppState, product_db::{delete_product_by_id, query_product_by_id, query_product_with_user, query_products, query_user_products, update_create_product, update_product_to_active, update_product_to_inactive, update_product_to_mongo}}, models::{product_models::{PaginationParams, Storage, UpdateCreateProductModel}, user_models::{ResponseUser, User}}};
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
                Json(serde_json::json!({ "status": "success", "productId": &product.id, "message": "Produto criado com sucesso!" }))
            )
        } ,
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "status": "error", "message": err.to_string() }))
        )
    }

}

#[tracing::instrument]
pub async fn upload_product(
    State(state): State<Arc<AppState>>,mut multipart: Multipart  
) -> impl IntoResponse {
    let mut data_final: UpdateCreateProductModel = UpdateCreateProductModel::default();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name =field.name().unwrap_or_default();

        match name {
            "id" => {
                data_final.id = field.text().await.unwrap_or_default();
            }
            "description" => {
                data_final.description = field.text().await.unwrap_or_default();
            }
            "pixType" => {
                data_final.pix_type = field.text().await.unwrap_or_default();
            }
            "pixKey" => {
                data_final.pix_key = field.text().await.unwrap_or_default();
            }
            "pictures" => {
                if let Some(filename) = field.file_name() {
                    let file_path = format!("./uploads/{}/{}", data_final.id, filename); 
                    let _folder = fs::create_dir_all(format!("./uploads/{}", data_final.id)); // Cria a estrutura de diretórios se não existir 
                    let data = field.bytes().await.unwrap();
                    fs::write(&file_path, &data).unwrap(); // Salva a imagem no disco 
                    data_final.images.push(file_path);
                }
            }
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({ "status": "error", "error": "Erro ao processar informações" })),
                );
            }
        }
        
    }

    if data_final.id.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "status": "error", "error": "Nenhum produto foi enviado para a solicitação" })),
        );
    } else {
        match  update_create_product(&state, data_final).await {
            Ok(result) => {
                if result.matched_count == 0 {
                    (
                        StatusCode::NOT_FOUND,
                        Json(serde_json::json!({
                            "status": "error",
                            "message": "Produto não encontrado"
                        })),
                    )
                } else {
                    (
                        StatusCode::OK,
                        Json(serde_json::json!({
                            "status": "success",
                            "message": "Produto atualizado com sucesso",
                            "modified": result.modified_count
                        })),
                    )
                }
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
}


#[tracing::instrument]
pub async fn get_user_products (
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>, 
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    info!(id);
    match query_user_products(&state, &id,params.limit,params.offset).await {
        Ok(products) if !products.is_empty() => {
        let parsed_products: Vec<Product> = products
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
pub async fn get_products(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>
) -> impl IntoResponse {
    match query_products(&state,params.limit,params.offset, params.title).await {
        Ok(products) if !products.is_empty() => {
        let parsed_products: Vec<Product> = products
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
pub async fn get_product_with_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>
) -> impl IntoResponse {
    match query_product_by_id(&state, &id).await {
        Ok(Some(doc)) => {
            let product: Product = bson::from_bson(bson::Bson::Document(doc)).unwrap();

            (StatusCode::OK, 
                Json(serde_json::json!({
                "status": "success",
                "products": product
            })))
        },
        Ok(None) => { 
            (StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "error",
                    "message": "Produto não encontrado"
                })))        }
        Err(err) => { 
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "status": "error",
                    "message": format!("Erro ao encontrar o produto: {}", err),
                })),
            )
        }
    }
}

pub async fn get_product_with_id_and_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>
) -> impl IntoResponse {
    match query_product_with_user(&state, &id).await {
        Ok(Some((product,user))) => {
            let final_product: Product = bson::from_bson(bson::Bson::Document(product)).unwrap();
            let final_user: ResponseUser = bson::from_bson(bson::Bson::Document(user)).unwrap();

            (StatusCode::OK, 
                Json(serde_json::json!({
                "status": "success",
                "products": final_product,
                "user": final_user
            })))
        },
        Ok(None) => { 
            (StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "error",
                    "message": "Produto não encontrado"
                })))        }
        Err(err) => { 
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "status": "error",
                    "message": format!("Erro ao encontrar o produto: {}", err),
                })),
            )
        }
    }
}
#[tracing::instrument]
pub async fn inactive_product_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>
) -> impl IntoResponse {
    
    match update_product_to_inactive(&state, &id).await {
        Ok(result) => {
            if result.matched_count == 0 {
                (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({
                        "status": "error",
                        "message": "Produto não encontrado"
                    })),
                )
            } else {
                (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "status": "success",
                        "message": "Produto desativado com sucesso",
                        "modified": result.modified_count
                    })),
                )
            }
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "message": format!("Erro ao desativar o produto: {}", err),
            })),
        )
    }
}

#[tracing::instrument]
pub async fn active_product_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>
) -> impl IntoResponse {
    
    match update_product_to_active(&state, &id).await {
        Ok(result) => {
            if result.matched_count == 0 {
                (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({
                        "status": "error",
                        "message": "Produto não encontrado"
                    })),
                )
            } else {
                (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "status": "success",
                        "message": "Produto ativado com sucesso",
                        "modified": result.modified_count
                    })),
                )
            }
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "message": format!("Erro ao ativar o produto: {}", err),
            })),
        )
    }
}

#[tracing::instrument]
pub async fn delete_product(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>
) -> impl IntoResponse {
    match delete_product_by_id(&state, &id).await {
        Ok(result) => {
            if result.deleted_count == 0 {
                (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({
                        "status": "error",
                        "message": "Produto não encontrado"
                    })),
                )
            } else {
                (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "status": "success",
                        "message": "Produto deletado com sucesso",
                        "deleted": result.deleted_count
                    })),
                )
            }
        } Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "message": format!("Erro ao deletar o produto: {}", err),
            })),
        )
    }
}

#[tracing::instrument]
pub async fn update_product(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Product>
) -> impl IntoResponse {
    match update_product_to_mongo(&state, &payload).await {
        Ok(result) => {
            if result.matched_count == 0 {
                (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({
                        "status": "error",
                        "message": "Produto não encontrado"
                    })),
                )
            } else {
                (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "status": "success",
                        "message": "Produto atualizado com sucesso",
                        "modified": result.modified_count
                    })),
                )
            }
        } Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "message": format!("Erro ao atualizar o produto: {}", err),
            })),
        )
    }
}
