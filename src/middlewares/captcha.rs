use crate::config::*;
use crate::utils::error::*;
use axum::{http::Request, middleware::Next, response::Response};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct CaptchaResponse {
    success: bool,
}

pub async fn handle<B>(req: Request<B>, next: Next<B>) -> Result<Response, Error> {
    if !*CAPTCHA_ENABLED {
        return Ok(next.run(req).await);
    }

    let key = match req
        .headers()
        .get("X-Captcha-Key")
        .and_then(|v| v.to_str().ok())
    {
        Some(k) => k,
        _ => return Err(Error::MissingHeader),
    };

    let client = reqwest::Client::new();
    let body = json!({
        "response": key,
        "secret": *CAPTCHA_TOKEN,
        "sitekey": *CAPTCHA_KEY
    });

    let res = client
        .post("https://hcaptcha.com/siteverify")
        .body(body.to_string())
        .send()
        .await;

    if let Ok(res) = res {
        if let Ok(captcha) = res.json::<CaptchaResponse>().await {
            if captcha.success {
                return Ok(next.run(req).await);
            }
        }
    }

    Err(Error::FailedCaptcha)
}
