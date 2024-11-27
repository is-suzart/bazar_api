use std::sync::Arc;

use mongodb::{bson::{Document, to_bson}, Collection};
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
