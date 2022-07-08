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

use axum::{handler::Handler, http::StatusCode, middleware, Router, Server};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::builder().format_timestamp(None).init();

    log::info!("Connecting to database...");
    database::postgres::connect().await;

    log::info!("Connecting to redis...");
    database::redis::connect().await;

    use middlewares::*;

    let app = routes::mount(Router::new())
        .route("/ws", axum::routing::get(gateway::upgrade))
        .layer(middleware::from_fn(auth::handle))
        .layer(middleware::from_fn(ratelimit::handle!(50, 1000 * 60)))
        .layer(cors::handle())
        .fallback((|| async { StatusCode::NOT_FOUND }).into_service());

    let address: SocketAddr = format!("0.0.0.0:{}", *config::PORT).parse().unwrap();

    log::info!("Listening on: {}", address.port());

    Server::bind(&address)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

pub mod tests {
    use once_cell::sync::Lazy;
    use tokio::runtime::Runtime;

    pub fn run<F: std::future::Future>(f: F) -> F::Output {
        static RT: Lazy<Runtime> = Lazy::new(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
        });

        RT.block_on(f)
    }

    #[cfg(test)]
    #[ctor::ctor]
    fn setup() {
        dotenv::dotenv().ok();
        env_logger::builder().format_timestamp(None).try_init().ok();
        run(super::database::postgres::connect());
    }
}
