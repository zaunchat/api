#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate async_trait;

pub mod config;
pub mod database;
pub mod extractors;
pub mod middlewares;
pub mod routes;
pub mod structures;
pub mod utils;

use axum::{handler::Handler, http::StatusCode, middleware, Router, Server};
use std::net::SocketAddr;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    log::info!("Connecting to database...");
    database::connect().await;

    log::info!("Run migration...");
    utils::migration::migrate().await;

    use middlewares::*;

    let app = routes::mount(Router::new())
        .layer(cors::handle())
        .layer(middleware::from_fn(auth::handle))
        .layer(middleware::from_fn(ratelimit::handle!(50, 1000 * 60)))
        .fallback((|| async { StatusCode::NOT_FOUND }).into_service());

    let address: SocketAddr = format!("0.0.0.0:{}", *config::PORT).parse().unwrap();

    log::info!("Listening on: {}", address.port());

    Server::bind(&address)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
