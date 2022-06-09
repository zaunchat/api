use crate::config;
use crate::utils::error::*;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct Captcha {
    pub success: bool,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Captcha {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if *config::CAPTCHA_ENABLED == false {
            return Outcome::Success(Captcha { success: true });
        }

        let key = req.headers().get_one("X-Captcha-Key");

        if let None = key {
            return Outcome::Failure((Status::BadRequest, Error::FailedCaptcha));
        }

        let client = reqwest::Client::new();
        let body = json!({
            "response": key,
            "secret": *config::CAPTCHA_TOKEN,
            "sitekey": *config::CAPTCHA_KEY
        });

        let res = client
            .post("https://hcaptcha.com/siteverify")
            .body(body.to_string())
            .send()
            .await;

        if let Ok(res) = res {
            if let Ok(captcha) = res.json::<Captcha>().await {
                if captcha.success {
                    return Outcome::Success(captcha);
                }
            }
        }

        return Outcome::Failure((Status::BadRequest, Error::FailedCaptcha));
    }
}

use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::{SecurityScheme, SecuritySchemeData};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};

impl<'r> OpenApiFromRequest<'r> for Captcha {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let mut requirements = schemars::Map::new();
        requirements.insert("captcha".to_owned(), vec![]);

        Ok(RequestHeaderInput::Security(
            "captcha".to_owned(),
            SecurityScheme {
                data: SecuritySchemeData::ApiKey {
                    name: "x-captcha-key".to_owned(),
                    location: "header".to_owned(),
                },
                description: Some("Recaptcha key to verify your request".to_owned()),
                extensions: schemars::Map::new(),
            },
            requirements,
        ))
    }
}
