use mongodb::{Client, options::ClientOptions, Database};
use std::env;

pub struct AppState {
    pub database: Database,
}

impl AppState {
    pub fn new(client: Client) -> Self {
        let db_name = env::var("MONGO_DB").unwrap_or_else(|_| "bazar_db".to_string());
        let database = client.database(&db_name);
        AppState { database }
    }
}

pub async fn create_mongo_client() -> mongodb::error::Result<Client> {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let mut client_options = ClientOptions::parse(&mongo_uri).await?;
    client_options.app_name = Some("Bazar Api".to_string());
    let client = Client::with_options(client_options)?;
    Ok(client)
}
