#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate lazy_static;

pub mod database;
pub mod routes;
pub mod structures;
pub mod utils;

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
