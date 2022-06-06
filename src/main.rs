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

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    log::info!("Connecting to database...");
    database::connect().await;

    let auth = fairings::auth::Auth {
        ignore: vec![
            "/".into(),
            "/auth/accounts/register".into(),
            "/auth/accounts/verify".into(),
            "/auth/sessions/login".into(),
            "/ratelimit",
        ],
    };

    let rocket = rocket::build();

    let _ = routes::mount(rocket)
        // Global Rate limit is 50 requests per 5 seconds
        .attach(ratelimit::RateLimit::new(50, 1000 * 5))
        .attach(auth)
        .mount("/", ratelimit::routes())
        .mount("/", auth::routes())
        .launch()
        .await;
}
