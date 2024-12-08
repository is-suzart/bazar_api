use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::helpers;


#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub user_id: String,
    pub active: bool,
    pub info: ProductInfo,
    pub images: Vec<String>,
    pub storage: Storage,
    pub product_cards: Option<Vec<ProductCard>>,
    pub created_at: String,
    pub updated_at: String,
    pub custom_option: HashMap<String, String>
}
#[derive(Serialize, Deserialize)]
pub struct ProductInfo {
    pub title: String,
    pub desc: String,
    pub subtitle: String,
    pub price: String,
    pub promotional_price: String,
    pub product_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Storage {
    pub total: u64,
    pub avaliable: u64,
    pub saled: u64
}
#[derive(Serialize, Deserialize)]
pub struct ProductCard {
    pub title: String,
    pub desc: String,
    image: String
}

impl Product {
    pub fn new(user_id: String,info: ProductInfo, storage: Storage) -> Self {
        let timestamp_brazil = helpers::timezone::get_current_timezone();
        let final_id = Uuid::new_v4();
        Product {
            id: final_id.to_string(),
            user_id,
            active: true,
            info,
            images: Vec::new(),
            storage,
            product_cards: None,
            created_at: timestamp_brazil.to_rfc3339().to_string(),
            updated_at: timestamp_brazil.to_rfc3339().to_string(),
            custom_option: HashMap::new()
        }
    }
}