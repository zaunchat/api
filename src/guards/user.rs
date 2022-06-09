use crate::structures::User;
use crate::utils::error::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = req.headers().get_one("Authorization");

        if token.is_none() {
            return Outcome::Failure((Status::BadRequest, Error::InvalidToken));
        }

        let user = req
            .local_cache_async(User::fetch_by_token(token.unwrap()))
            .await;

        if let Ok(user) = user {
            return Outcome::Success(user.clone());
        } else {
            return Outcome::Failure((Status::BadRequest, Error::InvalidToken));
        }
    }
}

use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::{SecurityScheme, SecuritySchemeData};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};

impl<'r> OpenApiFromRequest<'r> for User {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let mut requirements = schemars::Map::new();
        requirements.insert("token".to_owned(), vec![]);

        Ok(RequestHeaderInput::Security(
            "token".to_owned(),
            SecurityScheme {
                data: SecuritySchemeData::ApiKey {
                    name: "authorization".to_owned(),
                    location: "header".to_owned(),
                },
                description: Some("Session authentication token".to_owned()),
                extensions: schemars::Map::new(),
            },
            requirements,
        ))
    }
}
