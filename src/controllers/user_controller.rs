use axum::{extract::{State, Json}, http::StatusCode, response::IntoResponse};
use crate::{db::{mongo::AppState, user_db::insert_user}, helpers::token::generate_jwt, models::user_models::Response};
use crate:: models::user_models::{CreateUserModel, User};
use std::sync::Arc;
use crate::helpers::password;


pub async fn create_user(
    State(state): State<Arc<AppState>>, Json(payload): Json<CreateUserModel>,  // Recebe o payload da requisição
) -> impl IntoResponse {  // Especifica que a função implementa IntoResponse
    // Criação do usuário com os dados recebidos
    let (hashed_password, salt) = password::hash(&payload.password);
    let user = User::new(
        payload.name,
        payload.email,
        hashed_password,
        salt,
        payload.state,
        payload.city,
        payload.role,
    );

    match insert_user(&state, &user).await {
        Ok(_insert_result) => {
            let token = generate_jwt(&user.id.to_string());
            // Retorna o status de criação e o usuário como JSON
            (StatusCode::CREATED, Json(Response::Success { status: _insert_result.inserted_id.to_string(), id: Some(user.id), token }))
        },
        Err(err) => {
            // Caso ocorra erro, retorna um erro genérico em formato JSON
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Response::Error { status:err.to_string(), message: String::from("Erro ao criar usuário!") }))
        }
    }

}
