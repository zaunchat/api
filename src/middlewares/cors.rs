use axum::http::header::*;
use axum::http::Method;
use std::str::FromStr;
use tower_http::cors::{Any, CorsLayer};

pub fn handle() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::DELETE,
            Method::GET,
            Method::HEAD,
            Method::OPTIONS,
            Method::PATCH,
            Method::POST,
            Method::PUT,
        ])
        .allow_headers([
            ACCEPT,
            AUTHORIZATION,
            CONTENT_LENGTH,
            CONTENT_TYPE,
            HeaderName::from_str("X-Captcha-Key").unwrap(),
        ])
}
