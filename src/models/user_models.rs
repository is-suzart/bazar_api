use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;


#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub created_at: String,
    pub updated_at: String,
    pub state: String,
    pub city: String,
    pub profile_picture: String,
    pub role: String,
}

impl User {
    pub fn new(name: String, email: String, password: String, salt: String, state: String, city: String, role: String) -> Self {
        let timestamp = Utc::now().to_rfc3339(); // Pega o horário atual em formato ISO
        let final_id = Uuid::new_v4();
        User {
            id: final_id.to_string(), // Gera um ID único
            name,
            email,
            password,
            salt,
            created_at: timestamp.clone(),
            updated_at: timestamp,
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
    pub created_at: String,
    pub updated_at: String,
    pub state: String,
    pub city: String,
    pub profile_picture: String,
    pub role: String,
}

impl ResponseUser {
    pub fn new(id:String,name: String, email: String,created_at:String,updated_at:String,
        state: String, city: String, profile_picture:String, role: String) -> Self {

        ResponseUser {
            id,
            name,
            email,
            created_at,
            updated_at,
            state,
            city,
            profile_picture,
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


