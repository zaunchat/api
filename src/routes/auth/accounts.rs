use crate::config::*;
use crate::guards::captcha::Captcha;
use crate::guards::r#ref::Ref;
use crate::structures::*;
use crate::utils::email;
use crate::utils::error::*;
use argon2::Config;
use rocket::serde::{json::Json, Deserialize};
use validator::Validate;

#[derive(Deserialize, Validate, JsonSchema)]
pub struct RegisterSchema {
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String,
    #[validate(email)]
    pub email: String,
    pub invite_code: Option<String>,
}

#[derive(Deserialize, Validate, JsonSchema)]
pub struct RequestInvite {
    pub email: String,
}

#[openapi]
#[post("/register", data = "<data>")]
async fn register(_captcha: Captcha, data: Json<RegisterSchema>) -> Result<Json<User>> {
    let mut data = data.into_inner();

    data.validate()
        .map_err(|error| Error::InvalidBody { error })?;

    data.email = email::normalise(data.email);

    let invite = if *REQUIRE_INVITE_TO_REGISTER && data.invite_code.is_some() {
        email::Invite::find_one(|q| q.eq("code", data.invite_code.as_ref().unwrap())).await
    } else {
        None
    };

    if *REQUIRE_INVITE_TO_REGISTER {
        if let Some(invite) = &invite {
            if invite.used {
                return Err(Error::InviteAlreadyTaken);
            }
        } else {
            return Err(Error::RequireInviteCode);
        }
    }

    if User::email_taken(&data.email).await {
        return Err(Error::EmailAlreadyInUse);
    }

    let config = Config::default();
    let salt = nanoid::nanoid!(24);
    let hashed_password =
        argon2::hash_encoded(data.password.as_bytes(), salt.as_bytes(), &config).unwrap();

    let mut user = User::new(data.username, data.email, hashed_password);

    if *EMAIL_VERIFICATION && email::send(&user).await {
        log::debug!("Email have been sent to: {}", user.email);
    } else {
        user.verified = true;
    }

    if let Some(mut invite) = invite {
        invite.taken_by = user.id.into();
        invite.used = true;
        invite.update().await;
    }

    user.save().await;

    Ok(Json(user))
}

#[openapi]
#[get("/verify/<user_id>/<code>")]
async fn verify(user_id: Ref, code: &str) -> Result<()> {
    if email::verify(user_id.0, code).await {
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
