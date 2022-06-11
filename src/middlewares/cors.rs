use axum::http::Method;
use tower_http::cors::CorsLayer;

pub fn handle() -> CorsLayer {
    CorsLayer::new().allow_methods([
        Method::GET,
        Method::DELETE,
        Method::PATCH,
        Method::POST,
        Method::OPTIONS,
        Method::HEAD,
    ])
}
