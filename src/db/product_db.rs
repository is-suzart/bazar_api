use std::sync::Arc;

use crate::db::mongo::AppState;
use crate::models::product_models::Product;
use mongodb::{bson::{doc, to_bson, Document}, options::FindOneOptions, Collection};
use mongodb::results::InsertOneResult;

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