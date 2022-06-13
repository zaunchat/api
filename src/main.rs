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
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;

#[tokio::main]
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

    if *config::HTTPS_ENABLED {
        utils::ssl::request()
            .await
            .expect("Couldn't get the certificate for https");

        let config = RustlsConfig::from_pem_file("public.pem", "private.pem")
            .await
            .unwrap();

        axum_server::bind_rustls(address, config)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    } else {
        Server::bind(&address)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    }
}
