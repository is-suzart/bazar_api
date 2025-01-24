use mongodb::bson::{doc, Document};
use mongodb::options::AggregateOptions;
use mongodb::{Collection, error::Result};

async fn get_favorite_products(
    favorites_collection: Collection<Document>,
    user_id: &str,
) -> Result<Vec<Document>> {
    let pipeline = vec![
        doc! { "$match": { "userId": user_id } }, // Filtra os favoritos do usuário
        doc! { 
            "$lookup": {
                "from": "products", // Nome da coleção de produtos
                "localField": "productId", // Campo na coleção de favoritos
                "foreignField": "id", // Campo correspondente na coleção de produtos
                "as": "product" // Nome do campo para resultados combinados
            }
        },
        doc! { "$unwind": "$product" }, // Separa cada produto favorito em documentos individuais
        doc! { "$replaceRoot": { "newRoot": "$product" } } // Substitui o documento pelo conteúdo do produto
    ];

    let mut cursor = favorites_collection.aggregate(pipeline).await?;
    let results: Vec<Document> = cursor.try_collect().await?;
    Ok(results)
}