use std::str::FromStr;

use axum::http::header::*;
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

pub fn handle() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::DELETE,
            Method::PATCH,
            Method::POST,
            Method::OPTIONS,
            Method::HEAD,
        ])
        .allow_headers([
            ACCEPT,
            AUTHORIZATION,
            CONTENT_TYPE,
            CONTENT_LENGTH,
            HeaderName::from_str("X-Captcha-Key").unwrap(),
        ])
}
