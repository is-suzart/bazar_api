use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;

// Payload do token
#[derive(Serialize)]
struct Claims {
    sub: String, // ID do usuário
    exp: usize,  // Timestamp de expiração
}

// Função para gerar o token JWT
pub fn generate_jwt(user_id: &str) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7)) // Expira em 1 hora
        .expect("Falha ao calcular a expiração")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secretaqui".as_ref()), // Substitua por uma chave segura
    )
    .expect("Falha ao gerar o token")
}
