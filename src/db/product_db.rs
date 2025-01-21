use std::sync::Arc;

use crate::{db::mongo::AppState, models::product_models::UpdateCreateProductModel};
use crate::models::product_models::Product;
use futures::StreamExt;
use mongodb::options::FindOneOptions;
use mongodb::Cursor;
use mongodb::{bson::{doc, to_bson, Document}, Collection};
use mongodb::results::{InsertOneResult, UpdateResult};

pub async fn insert_product (
    state: &Arc<AppState>,
    product: &Product
) -> mongodb::error::Result<InsertOneResult> {
    let collection: Collection<Document> = state.database.collection("products");
    let product_doc = to_bson(product)?
        .as_document()
        .unwrap()
        .clone();
    collection.insert_one(product_doc).await
}

pub async fn update_create_product (
    state: &Arc<AppState>,
    data: UpdateCreateProductModel
) -> mongodb::error::Result<UpdateResult> {
    let collection: Collection<Document> = state.database.collection("products");

    collection
        .update_one(
            doc! { "id": data.id }, // Localiza o produto pelo ID
            doc! {
                "$set": {
                    "description": data.description,
                    "pix_type": data.pix_type,
                    "pix_key": data.pix_key,
                    "images": data.images
                }
            }
        )
        .await
}

pub async fn query_user_products(
    app_state: &Arc<AppState>,
    id: &String,
    limit: Option<i64>, 
    offset: Option<u64>
) -> mongodb::error::Result<Vec<Document>> {
    let collection: Collection<Document> = app_state.database.collection("products");
    let filter = doc! { "user_id": id };
    // Busca múltiplos documentos
    let mut cursor: Cursor<Document> = collection.find(filter).limit(limit.unwrap_or(10)).skip(offset.unwrap_or(0)).await?;
    let mut results = Vec::new();

    // Itera sobre os documentos encontrados
    while let Some(doc) = cursor.next().await {
        results.push(doc?); // Adiciona o documento ao vetor
    }

    Ok(results)
}

pub async fn query_products(
    app_state: &Arc<AppState>,
    limit: Option<i64>,
    offset: Option<u64>,
    title: Option<String>
) -> mongodb::error::Result<Vec<Document>> {
    let collection: Collection<Document> = app_state.database.collection("products");
    let filter = match title {
        Some(title) => doc! {
            "info.title": { "$regex": title, "$options": "i" }
        },
        None => doc! {}
    };
    let mut cursor: Cursor<Document> = collection.find(filter).limit(limit.unwrap_or(10)).skip(offset.unwrap_or(0)).await?;
    let mut results = Vec::new();

    // Itera sobre os documentos encontrados
    while let Some(doc) = cursor.next().await {
        results.push(doc?); // Adiciona o documento ao vetor
    }

    Ok(results)
}



pub async fn query_product_by_id(
    app_state: &Arc<AppState>,
    id: &String
) -> mongodb::error::Result<Option<Document>> {
    let collection: Collection<Document> = app_state.database.collection("products");
    let options = FindOneOptions::builder()
    .build();
    let doc = collection.find_one(doc! { "id": id}).with_options(options).await?;
    Ok(doc)


}