#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate opg;
#[macro_use]
extern crate serde_with;

pub mod config;
pub mod database;
pub mod extractors;
pub mod gateway;
pub mod middlewares;
pub mod routes;
pub mod structures;
pub mod utils;

use axum::{middleware, routing::get, Router, Server};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    env_logger::Builder::from_env(env_logger::Env::default().filter_or("RUST_LOG", "info"))
        .format_timestamp(None)
        .init();

    log::info!("Connecting to database...");
    database::postgres::connect().await;

    log::info!("Connecting to redis...");
    database::redis::connect().await;

    use middlewares::*;

    let app = routes::mount(Router::new())
        .route("/ws", get(gateway::upgrade))
        .layer(middleware::from_fn(auth::handle))
        .layer(middleware::from_fn(ratelimit::handle!(50, 1000 * 60)))
        .layer(cors::handle())
        .layer(compression::handle());

    let addr = SocketAddr::from(([0, 0, 0, 0], *config::PORT));

    log::info!("Listening on: {}", addr.port());

    Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use log::LevelFilter;
    use once_cell::sync::Lazy;
    use tokio::runtime::{Builder, Runtime};

    static RUNTIME: Lazy<Runtime> =
        Lazy::new(|| Builder::new_multi_thread().enable_all().build().unwrap());

    pub fn run<F: std::future::Future>(f: F) -> F::Output {
        RUNTIME.block_on(f)
    }

    #[ctor::ctor]
    fn setup() {
        dotenv::dotenv().ok();

        env_logger::builder()
            .filter_level(LevelFilter::Debug)
            .parse_filters("fred=off")
            .format_timestamp(None)
            .init();

        run(database::postgres::connect());
        run(database::redis::connect());
    }
}
