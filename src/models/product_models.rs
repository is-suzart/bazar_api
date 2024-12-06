use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub user_id: String,
    pub active: bool,
    pub info: ProductInfo,
    pub images: Vec<String>,
    pub storage: Storage,
    pub product_cards: Vec<ProductCard>,
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