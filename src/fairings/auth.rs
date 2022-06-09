use crate::structures::User;
use crate::utils::error::*;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{uri::Origin, Method},
    outcome::Outcome,
    Data, Request, Route,
};

fn is_ignored(path: &str) -> bool {
    match path {
        "/" => true,
        "/auth/accounts/register" => true,
        "/auth/accounts/verify" => true,
        "/auth/sessions/login" => true,
        "/ratelimit" => true,
        _ => false
    }
}

pub struct Auth;

#[rocket::async_trait]
impl Fairing for Auth {
    fn info(&self) -> Info {
        Info {
            name: "Authentication",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        let path = req.uri().path().as_str();

        if is_ignored(&path) {
            return
        }

        if let Outcome::Failure(_) = req.guard::<User>().await {
            req.set_method(Method::Get);
            req.set_uri(Origin::parse("/unauthorized").unwrap())
        }
    }
}

pub fn new() -> Auth {
    Auth {}
}

#[get("/unauthorized")]
fn unauthorized() -> Result<()> {
    Err(Error::Unauthorized)
}

pub fn routes() -> Vec<Route> {
    routes![unauthorized]
}
