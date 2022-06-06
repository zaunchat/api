use crate::utils::error::*;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{uri::Origin, Method},
    outcome::Outcome,
    Data, Request, Route,
};


pub struct RateLimiter;

#[rocket::async_trait]
impl Fairing for RateLimiter<'static> {
    fn info(&self) -> Info {
        Info {
            name: "RateLimiter",
            kind: Kind::Request
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        todo!();
        req.set_method(Method::Get);
        req.set_uri(Origin::parse("/rate_limited").unwrap())
    }
}


#[get("/rate_limited")]
fn rate_limited() -> String {
    "Rate limited".into()
}

pub fn routes() -> Vec<Route> {
    routes![rate_limited]
}