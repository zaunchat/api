#[macro_use]
extern crate rbatis;

pub mod utils;
pub mod database;
pub mod structures;
pub mod guards;
pub mod routes;

#[async_std::main]
fn main() {
    db::connect().await;

    let rocket = rocket::build();
}