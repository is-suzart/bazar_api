use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::helpers;


#[derive(Serialize, Deserialize,Debug)]
pub struct Product {
    pub id: String,
    pub user_id: String,
    pub active: bool,
    pub info: ProductInfo,
    pub images: Vec<String>,
    pub storage: Storage,
    pub created_at: String,
    pub updated_at: String,
    pub custom_option: HashMap<String, String>,
    pub description: String,
    pub pix_type: String,
    pub pix_key: String
}

#[derive(Serialize, Deserialize)]
pub struct UpdateCreateProductModel {
    pub id: String,
    pub description: String,
    pub pix_type: String,
    pub images: Vec<String>,
    pub pix_key: String
}

impl UpdateCreateProductModel {
    pub fn default () -> Self {
        UpdateCreateProductModel {
            id: String::from(""),
            description: String::from(""),
            pix_type: String::from(""),
            images: Vec::new(),
            pix_key: String::from(""),
        }
    }
}

#[derive(Debug,Serialize, Deserialize)]
pub struct ProductInfo {
    pub title: String,
    pub subtitle: String,
    pub price: String,
    pub have_promotion: bool,
    pub promotional_price: Option<String>,
    pub promotional_amount: Option<u64>,
    pub product_type: String,
    pub place: Option<String>
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Storage {
    pub total: u64,
    pub avaliable: u64,
    pub saled: u64
}

impl Storage {
    pub fn new (value: u64) -> Self {
        Storage {
            avaliable: value,
            total:  value,
            saled: 0
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct ProductCard {
    pub title: String,
    pub desc: String,
    image: String
}

impl Product {
    pub fn new(user_id: String, info: ProductInfo, storage: Storage) -> Self {
        let timestamp_brazil = helpers::timezone::get_current_timezone();
        let final_id = Uuid::new_v4();
        Product {
            id: final_id.to_string(),
            user_id,
            active: true,
            info,
            images: Vec::new(),
            storage,
            created_at: timestamp_brazil.to_rfc3339().to_string(),
            updated_at: timestamp_brazil.to_rfc3339().to_string(),
            custom_option: HashMap::new(),
            description: "".to_string(),
            pix_type: "".to_string(),
            pix_key: "".to_string(),
        }
    }
}


#[derive(Debug,Serialize, Deserialize)]
pub struct CreateProductModel {
    pub user_id: String,
    pub info: ProductInfo,
    pub storage: u64,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct PaginationParams {
    pub limit: Option<i64>,
    pub offset: Option<u64>,
    pub title: Option<String>
}

#[derive(Debug,Serialize, Deserialize)]
pub struct FavoriteProduct {
    pub user_id: String,
    pub product_id: String
}

// pub struct FrontProductModel {
//     pub user_id: String,
//     pub subtitle: String,
//     pub name: String,
//     pub price: String,
//     pub isPromo: bool,
//     pub promotionalAmount: Option<u64>, // Rust's equivalent to Dart's nullable int
//     pub promotionalPrice: Option<String>, // Rust's equivalent to Dart's nullable String
//     pub productType: String, // 'type' is a reserved keyword in Rust, so we use 'r#type' to escape it
//     pub place: Option<String>, // Rust's equivalent to Dart's nullable String
// }
    