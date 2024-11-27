use std::sync::Arc;

use mongodb::{bson::{doc, Document, to_bson}, Collection};
use crate::{models::user_models::User, AppState};
use mongodb::results::InsertOneResult;

pub async fn insert_user(
    app_state: &Arc<AppState>, 
    user: &User
) -> mongodb::error::Result<InsertOneResult> {
    // Acessa a coleção 'users' no banco de dados
    let collection: Collection<Document> = app_state.database.collection("users");

    // Converte o User para BSON e depois para Document
    let user_doc = to_bson(user)?
        .as_document() // Converte BSON para Document
        .unwrap()
        .clone(); // Clona o Document para garantir que ele seja independente

    // Insere o usuário no banco de dados
    collection.insert_one(user_doc).await
}

pub async fn query_user_with_id(
    app_state: &Arc<AppState>,
    id: &String
) -> mongodb::error::Result<Option<Document>> {
    let collection: Collection<Document> = app_state.database.collection("users");
    let doc = collection.find_one(doc! { "id": id }).await?;
    Ok(doc)


}

pub async fn query_user_by_email(
    app_state: &Arc<AppState>,
    email: &String
) -> mongodb::error::Result<Option<Document>> {
    let collection: Collection<Document> = app_state.database.collection("users");
    let doc = collection.find_one(doc! { "email": email }).await?;
    Ok(doc)
}