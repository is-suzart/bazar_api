use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;


#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
    pub state: String,
    pub city: String,
    pub profile_picture: String,
    pub role: String,
}

impl User {
    pub fn new(name: String, email: String, password: String, state: String, city: String, role: String) -> Self {
        let timestamp = Utc::now().to_rfc3339(); // Pega o horário atual em formato ISO
        let final_id = Uuid::new_v4();
        User {
            id: final_id.to_string(), // Gera um ID único
            name,
            email,
            password,
            created_at: timestamp.clone(),
            updated_at: timestamp,
            state,
            city,
            profile_picture: String::from(""), // Perfil vazio por padrão
            role,
        }
    }
}


#[derive(Deserialize)]
pub struct CreateUserModel {
    pub name: String,
    pub email: String,
    pub password: String,
    pub state: String,
    pub city: String,
    pub role: String,
}