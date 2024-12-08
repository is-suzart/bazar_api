use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::helpers;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub telephone: String,
    pub salt: String,
    pub created_at: String,
    pub updated_at: String,
    pub state: String,
    pub city: String,
    pub profile_picture: String,
    pub role: String,
}

impl User {
    pub fn new(name: String, email: String, password: String, telephone: String, salt: String, state: String, city: String, role: String) -> Self {
        let timestamp_brazil = helpers::timezone::get_current_timezone();
        let final_id = Uuid::new_v4();
        User {
            id: final_id.to_string(), // Gera um ID único
            name,
            email,
            password,
            telephone,
            salt,
            created_at: timestamp_brazil.to_rfc3339().to_string(),
            updated_at: timestamp_brazil.to_rfc3339().to_string(),
            state,
            city,
            profile_picture: String::from(""), // Perfil vazio por padrão
            role,
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub telephone: String,
    pub created_at: String,
    pub updated_at: String,
    pub state: String,
    pub city: String,
    pub profile_picture: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct CreateUserModel {
    pub name: String,
    pub email: String,
    pub password: String,
    pub telephone: String,
    pub state: String,
    pub city: String,
    pub role: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginUserModel {
    pub email: String,
    pub password: String,
}


