use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Response {
    Success { status: String, id: Option<String>, token: String },
    Error { status: String, message: String },
}