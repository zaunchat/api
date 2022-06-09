#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate lazy_static;

pub mod config;
pub mod database;
pub mod fairings;
pub mod guards;
pub mod routes;
pub mod structures;
pub mod utils;

use fairings::*;

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();
    env_logger::init();

    log::info!("Connecting to database...");
    database::connect().await;

    log::info!("Run migration...");
    utils::migration::migrate().await;

    let auth = fairings::auth::Auth {
        ignore: vec![
            "/",
            "/auth/accounts/register",
            "/auth/accounts/verify",
            "/auth/sessions/login",
            "/ratelimit",
        ],
    };

    let rocket = rocket::build();

    routes::mount(rocket)
        .attach(ratelimit::RateLimiter)
        .attach(auth)
        .mount("/", ratelimit::routes())
        .mount("/", auth::routes())
}
