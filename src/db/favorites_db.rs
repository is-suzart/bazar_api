use std::sync::Arc;
use crate::db::mongo::AppState;
use crate::models::product_models::FavoriteProduct;
use bson::to_bson;
use futures::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::Collection;

pub async fn get_favorite_products(
    state: &Arc<AppState>,
    user_id: &String,
) -> mongodb::error::Result<Vec<Document>> {
    let collection: Collection<Document> = state.database.collection("favorites");
    let pipeline = vec![
        doc! { "$match": { "user_id": user_id } }, // Filtra os favoritos do usuário
        doc! { 
            "$lookup": {
                "from": "products", // Nome da coleção de produtos
                "localField": "product_id", // Campo na coleção de favoritos
                "foreignField": "id", // Campo correspondente na coleção de produtos
                "as": "product" // Nome do campo para resultados combinados
            }
        },
        doc! { "$unwind": "$product" }, // Separa cada produto favorito em documentos individuais
        doc! { "$replaceRoot": { "newRoot": "$product" } } // Substitui o documento pelo conteúdo do produto
    ];

    let cursor = collection.aggregate(pipeline).await?;
    let results: Vec<Document> = cursor.try_collect().await?;
    Ok(results)
}

pub async fn post_favorite(
    state: &Arc<AppState>,
    favorite: &FavoriteProduct,
) -> mongodb::error::Result<()> {
    let collection: Collection<Document> = state.database.collection("favorites");
    let favorite_doc = to_bson(favorite)?
        .as_document()
        .unwrap()
        .clone();
    collection.insert_one(favorite_doc).await?;
    Ok(())
}

pub async fn delete_favorite(
    state: &Arc<AppState>,
    user_id: &String,
    product_id: &String,
) -> mongodb::error::Result<()> {
    let collection: Collection<Document> = state.database.collection("favorites");
    collection.delete_one(doc! { "user_id": user_id, "product_id": product_id }).await?;
    Ok(())
}
pub async fn get_favorite_by_id(
    state: &Arc<AppState>,
    user_id: &String,
    product_id: &String,
) -> mongodb::error::Result<Option<Document>> {
    let collection: Collection<Document> = state.database.collection("favorites");
    let doc = collection.find_one(doc! { "user_id": user_id, "product_id": product_id }).await?;
    Ok(doc)
}