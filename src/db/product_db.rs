use std::sync::Arc;

use crate::{db::mongo::AppState, models::product_models::UpdateCreateProductModel};
use crate::models::product_models::Product;
use futures::{StreamExt, TryStreamExt};
use mongodb::options::FindOneOptions;
use mongodb::Cursor;
use mongodb::{bson::{doc, to_bson, Document}, Collection};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};

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
            "info.title": { "$regex": title, "$options": "i","active": true }
        },
        None => doc! {"active": true}
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

pub async fn query_product_with_user(
    app_state: &Arc<AppState>,
    product_id: &String,
) -> mongodb::error::Result<Option<(Document, Document)>> {
    let collection: Collection<Document> = app_state.database.collection("products");

    let pipeline = vec![
        doc! { "$match": { "id": product_id } }, // Filtra o produto pelo ID
        doc! {
            "$lookup": {
                "from": "users", // Nome da coleção de usuários
                "localField": "user_id", // Campo no produto que referencia o usuário
                "foreignField": "id", // Campo correspondente na coleção de usuários
                "as": "user" // Nome do campo onde os dados do usuário serão armazenados
            }
        },
        doc! { "$unwind": "$user" }, // Garante que os dados do usuário não estejam em um array
        doc! { 
            "$project": {
                "user.password": 0, // Exclui o campo "password" do usuário
                "user.salt": 0,     // Exclui o campo "salt" do usuário
            }
        }
    ];

    let mut cursor = collection.aggregate(pipeline).await?;
    if let Some(doc) = cursor.try_next().await? {
        // Extrai os dados do produto e do usuário
        if let Ok(user) = doc.get_document("user") {
            let user = user.clone(); // Clona o documento do usuário
            let mut product = doc.clone(); // Clona o documento do produto
            product.remove("user"); // Remove o campo "user" do produto para evitar redundância
            return Ok(Some((product, user)));
        }
    }
    Ok(None) // Retorna None se nenhum documento for encontrado
}



pub async fn update_product_to_inactive(
    app_state: &Arc<AppState>,
    id: &String
) -> mongodb::error::Result<UpdateResult> {
    let collection: Collection<Document> = app_state.database.collection("products");
    collection
        .update_one(
            doc! { "id": id },
            doc! {
                "$set": {
                    "active": false
                }
            }
        )
        .await
}

pub async fn delete_product_by_id(
    app_state: &Arc<AppState>,
    id: &String
) -> mongodb::error::Result<DeleteResult> {
    let collection: Collection<Document> = app_state.database.collection("products");
    collection
        .delete_one(
            doc! { "id": id },
        )
        .await
}
pub async fn update_product_to_mongo(
    app_state: &Arc<AppState>,
    product: &Product
) -> mongodb::error::Result<UpdateResult> {
    let collection: Collection<Document> = app_state.database.collection("products");
    let product_doc = to_bson(product)?
        .as_document()
        .unwrap()
        .clone();
    collection.update_one(doc! { "id": &product.id }, product_doc).await
}

pub async fn update_product_to_active(
    app_state: &Arc<AppState>,
    id: &String
) -> mongodb::error::Result<UpdateResult> {
    let collection: Collection<Document> = app_state.database.collection("products");
    collection
        .update_one(
            doc! { "id": id },
            doc! {
                "$set": {
                    "active": true
                }
            }
        )
        .await
}