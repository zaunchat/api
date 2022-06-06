use crate::structures::User;
use crate::utils::error::Error;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = req.headers().get_one("x-session-token");

        if token.is_none() {
            return Outcome::Failure((Status::BadRequest, Error::InvalidToken));
        }

        let user = req.local_cache_async(User::fetch_by_token(&token.unwrap())).await;

        if let Ok(user) = user {
            return Outcome::Success(user.clone());
        } else {
            return Outcome::Failure((Status::BadRequest, Error::InvalidToken));
        }
    }
}
