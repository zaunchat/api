use crate::config::EMAIL_VERIFICATION;
use crate::guards::captcha::Captcha;
use crate::guards::r#ref::Ref;
use crate::structures::*;
use crate::utils::email;
use crate::utils::error::*;
use argon2::Config;
use rocket::serde::{json::Json, Deserialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Clone, Copy, JsonSchema)]
pub struct RegisterSchema<'r> {
    #[validate(length(min = 3, max = 32))]
    pub username: &'r str,
    #[validate(length(min = 8, max = 32))]
    pub password: &'r str,
    #[validate(email)]
    pub email: &'r str,
}

#[openapi]
#[post("/register", data = "<data>")]
async fn register(_captcha: Captcha, data: Json<RegisterSchema<'_>>) -> Result<Json<User>> {
    let data = data.into_inner();

    data.validate()
        .map_err(|error| Error::InvalidBody { error })?;

    let email_in_use = User::find_one(|q| q.eq("email", &data.email))
        .await
        .is_some();

    if email_in_use {
        return Err(Error::EmailAlreadyInUse);
    }

    let config = Config::default();
    let salt = nanoid::nanoid!(24);
    let hashed_password = argon2::hash_encoded(
        data.password.to_string().as_bytes(),
        salt.as_bytes(),
        &config,
    )
    .unwrap();

    let mut user = User::new(data.username.into(), data.email.into(), hashed_password);

    match *EMAIL_VERIFICATION {
        true if email::send(&user).await => {
            log::debug!("Email have been sent to: {}", user.email);
        } // If email sending failed for any reason just verify the account.
        _ => {
            user.verified = true;
        }
    };

    user.save().await;

    Ok(Json(user))
}

#[openapi]
#[get("/verify/<user_id>/<code>")]
async fn verify(user_id: Ref, code: &str) -> Result<()> {
    let verified = email::verify(user_id.0, code).await;

    if verified {
        let mut user = user_id.user().await?;
        user.verified = true;
        user.update().await;
        Ok(())
    } else {
        Err(Error::UnknownAccount)
    }
}

pub fn routes() -> (Vec<rocket::Route>, rocket_okapi::okapi::openapi3::OpenApi) {
    openapi_get_routes_spec![register, verify]
}
