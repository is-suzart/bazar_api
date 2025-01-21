use axum::{extract::{Json, Path, State,Multipart}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use crate::{db::{mongo::AppState, user_db::{insert_user, query_user_by_email, query_user_by_id}}, helpers::{password::verify, token::generate_jwt}, models::user_models::{LoginUserModel, ResponseUser}, response::user_response::{CreateUserResponse, QueryUserResponse}};
use crate:: models::user_models::{CreateUserModel, User};
use std::sync::Arc;
use crate::helpers::password;
use uuid::Uuid;
use std::{fs, path::Path as StdPath};

pub async fn create_user(
    State(state): State<Arc<AppState>>, Json(payload): Json<CreateUserModel>,  // Recebe o payload da requisição
) -> impl IntoResponse {  // Especifica que a função implementa IntoResponse
    // Criação do usuário com os dados recebidos
    let (hashed_password, salt) = password::hash(&payload.password);
    let role = payload.role.unwrap_or("user".to_string());   
    let user = User::new(
        payload.name,
        payload.email,
        hashed_password,
        payload.telephone,
        salt,
        payload.state,
        payload.city,
        role
    );

    match insert_user(&state, &user).await {
        Ok(_insert_result) => {
            let token = generate_jwt(&user.id.to_string());
            // Retorna o status de criação e o usuário como JSON
            (StatusCode::CREATED, Json(CreateUserResponse::Success { status: _insert_result.inserted_id.to_string(), id: Some(user.id), token }))
        },
        Err(err) => {
            // Caso ocorra erro, retorna um erro genérico em formato JSON
            (StatusCode::INTERNAL_SERVER_ERROR, Json(CreateUserResponse::Error { status:err.to_string(), message: String::from("Erro ao criar usuário!") }))
        }
    }

}
pub async fn get_user_with_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>
) -> impl IntoResponse {
    match query_user_by_id(&state, &id).await {
        Ok(Some(doc)) => {
            let user: ResponseUser = bson::from_bson(bson::Bson::Document(doc)).unwrap();

            (StatusCode::OK, Json(QueryUserResponse::Success { status: "success".to_string(), user }))
        },
        Ok(None) => { 
            (StatusCode::NOT_FOUND, Json(QueryUserResponse::NotFound { status: "not_found".to_string(), message: "Usuário não encontrado".to_string() }))
        }
        Err(err) => { 
            (StatusCode::INTERNAL_SERVER_ERROR,Json(QueryUserResponse::Error { status: err.to_string(), message: "Erro interno".to_string() }))
        }
    }
}

pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginUserModel>
) -> impl IntoResponse {
    match query_user_by_email(&state, &payload.email).await {
        Ok(Some(doc)) => {
            let user: User = bson::from_bson(bson::Bson::Document(doc)).unwrap();
            if verify(&payload.password, &user.password, &user.salt) {
                let token = generate_jwt(&user.id.to_string());
                (StatusCode::OK, Json(CreateUserResponse::Success { status: "success".to_string(), id: Some(user.id), token }))
            } else {
                (StatusCode::UNAUTHORIZED, Json(CreateUserResponse::Error { status: "unauthorized".to_string(), message: "Credenciais inválidas".to_string() }))
            }
        },
        Ok(None) => { 
            (StatusCode::NOT_FOUND, Json(CreateUserResponse::Error { status: "not_found".to_string(), message: "Usuário não encontrado".to_string() }))
        }
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR,Json(CreateUserResponse::Error { status: err.to_string(), message: "Erro interno".to_string() }))
        }
    }
}

pub async fn upload_profile_picture(
    Path(id): Path<String>,         // Extraindo o ID do path
    mut multipart: Multipart       // Recebendo o arquivo
) -> impl IntoResponse {
    let upload_dir = format!("{}/{}","./uploads",id);

    // Cria o diretório de upload se não existir
    if !StdPath::new(&upload_dir).exists() {
        fs::create_dir_all(&upload_dir).unwrap();
    }

    // Processa os arquivos enviados
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();

        // Valida se o arquivo é uma imagem
        if !content_type.starts_with("image/") {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Only images are allowed" })),
            );
        }

        // Gera um nome único para o arquivo
        let unique_name: String = format!("{}_{}.{}", id, Uuid::new_v4(), file_name.split('.').last().unwrap());
        let file_path: String = format!("{}/{}", &upload_dir, unique_name);

        // Salva o arquivo no sistema
        let data = field.bytes().await.unwrap();
        fs::write(&file_path, data).unwrap();

        // Retorna o caminho do arquivo e o ID associado
        return (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "file_path": file_path,
                "id": id,
            })),
        );
    }

    // Caso nenhum arquivo seja enviado
    (
        StatusCode::BAD_REQUEST,
        Json(json!({ "error": "No file was provided" })),
    )
}