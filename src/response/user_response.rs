use serde::{Deserialize, Serialize};

use crate::models::user_models::ResponseUser;

#[derive(Serialize, Deserialize)]
#[serde(untagged)] 
pub enum CreateUserResponse {
    Success { status: String, id: Option<String>, token: String },
    Error { status: String, message: String },
}
#[derive(Serialize, Deserialize)]
#[serde(untagged)] 
 pub enum QueryUserResponse {
    Success { status: String, user: ResponseUser },
    NotFound { status: String, message: String },
    Error { status: String, message: String  }
}
