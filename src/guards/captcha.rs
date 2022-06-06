use crate::utils::error::Error;
use crate::CAPTCHA_ENABLED;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::Deserialize;
use serde_json::json;
use std::env;

#[derive(Deserialize)]
pub struct Captcha {
    pub success: bool,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Captcha {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if *CAPTCHA_ENABLED == false {
            return Outcome::Success(Captcha { success: true });
        }

        let key = req.headers().get_one("X-Captcha-Key");

        if let None = key {
            return Outcome::Failure((Status::BadRequest, Error::FailedCaptcha));
        }

        let client = reqwest::Client::new();
        let body = json!({
            "response": key,
            "secret": env::var("CAPTCHA_TOKEN").expect("CAPTCHA_TOKEN is required"),
            "sitekey": env::var("CAPTCHA_KEY").expect("CAPTCHA_KEY is required")
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
