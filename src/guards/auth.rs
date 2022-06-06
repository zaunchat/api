use crate::structures::User;
use crate::utils::error::{Error, Result};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{uri::Origin, Method},
    outcome::Outcome,
    Data, Request, Route,
};

pub struct Auth<'a> {
    pub ignore: Vec<&'a str>,
}

#[rocket::async_trait]
impl Fairing for Auth<'static> {
    fn info(&self) -> Info {
        Info {
            name: "Authentication",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        let path = req.uri().path().as_str();

        for &url in &self.ignore {
            if url == path {
                return;
            }
        }

        if let Outcome::Failure(_) = req.guard::<User>().await {
            req.set_method(Method::Get);
            req.set_uri(Origin::parse("/unauthorized").unwrap())
        }
    }
}

#[get("/unauthorized")]
fn unauthorized() -> Result<()> {
    Err(Error::Unauthorized)
}

pub fn routes() -> Vec<Route> {
    routes![unauthorized]
}
