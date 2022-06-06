#[get("/")]
pub fn root() -> String {
    "Up".into()
}

pub mod auth;
