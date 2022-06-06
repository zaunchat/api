#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate lazy_static;

pub mod database;
pub mod guards;
pub mod routes;
pub mod structures;
pub mod utils;

use std::env;

lazy_static! {
    pub static ref SMTP_ENABLED: bool = env::var("SMTP_ENABLED").is_ok();
    pub static ref CAPTCHA_ENABLED: bool = env::var("CAPTCHA_ENABLED").is_ok();
}

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("Connecting to database...");

    database::connect().await;

    let _ = rocket::build()
        .mount("/", routes![routes::root])
        .launch()
        .await;
}
