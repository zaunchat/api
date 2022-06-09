pub use rocket_cors::catch_all_options_routes;
use rocket_cors::{AllowedOrigins, AllowedHeaders, Cors, CorsOptions};
use std::str::FromStr;

pub fn new() -> Cors {
    CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_methods: [
            "Get", "Post", "Patch", "Delete", "Options", "Head",
        ]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        ..Default::default()
    }
    .to_cors()
    .unwrap()
}