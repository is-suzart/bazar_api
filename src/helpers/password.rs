use argon2::{password_hash::{SaltString, PasswordHasher}, Argon2};
use rand::rngs::OsRng; // Gerador de números aleatórios seguro

pub fn hash(password: &str) -> (String, String) {
    // Gera um salt aleatório usando OsRng
    let salt = SaltString::generate(&mut OsRng);

    // Configuração padrão do Argon2
    let argon2 = Argon2::default();

    // Cria o hash da senha com o salt gerado
    // Cria o hash da senha com o salt gerado
    let hashed_password = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Erro ao hashear a senha")
        .to_string(); // Converte o hash para string

    (hashed_password, salt.to_string())
}
