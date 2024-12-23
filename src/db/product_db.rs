use std::sync::Arc;

use crate::{db::mongo::AppState, models::product_models::UpdateCreateProductModel};
use crate::models::product_models::Product;
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