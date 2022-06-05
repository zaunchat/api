use rocket::http::Status;
use rocket::request::{self, Outcome, Request, FromRequest};

#[derive(Debug)]
enum Error {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // TODO
    }
}
