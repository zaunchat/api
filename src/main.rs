#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate lazy_static;

pub mod database;
pub mod guards;
pub mod fairings;
pub mod routes;
pub mod structures;
pub mod utils;
pub mod config;


#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();

    log::info!("Connecting to database...");
    database::connect().await;

    let auth = fairings::auth::Auth {
        ignore: vec![
            "/".into(),
            "/auth/accounts/register".into(),
            "/auth/accounts/verify".into(),
            "/auth/sessions/login".into(),
        ],
    };

    let ratelimit = fairings::ratelimit::RateLimiter {
        // TODO:
    };

    let rocket = rocket::build();

    routes::mount(rocket)
        .mount("/", fairings::auth::routes())
        .mount("/", fairings::ratelimit::routes())
        .attach(ratelimit)
        .attach(auth)
        .launch()
        .await;
}
