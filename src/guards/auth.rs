use rocket::http::Status;
use rocket::request::{self, Outcome, Request, FromRequest};
use crate::structures::user::User;

#[derive(Debug)]
enum Error {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = req.headers().get_one("x-session-token");
        
        if token.is_none() {
            return Outcome::Failure((Status::BadRequest, Error::Invalid));
        }

        let user = User::fetch_by_token(&token).await;

        if let Some(user) = user {
            return Outcome::Success(user);
        } else {
            return Outcome::Failure((Status::BadRequest, Error::Invalid));
        }
    }
}
