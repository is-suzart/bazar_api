use axum::http;
use tower_http::cors::{Any, CorsLayer};
use http::Method;
use http::header::HeaderName;

pub fn cors_middleware() -> CorsLayer {
    CorsLayer::new()
    .allow_origin(Any) // Permite qualquer origem (use com cautela em produção)
    .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE]) // Métodos permitidos
    .allow_headers(vec![HeaderName::from_static("content-type"), HeaderName::from_static("authorization")]) // Cabeçalhos permitidos
}

