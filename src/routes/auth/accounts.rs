use crate::config::*;
use crate::extractors::*;
use crate::structures::*;
use crate::utils::email;
use crate::utils::error::*;
use crate::utils::r#ref::Ref;
use argon2::Config;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, utoipa::Component)]
pub struct RegisterAccountOptions {
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String,
    #[validate(email)]
    pub email: String,
    pub invite_code: Option<String>,
}

#[utoipa::path(
    post,
    path = "/auth/accounts/register",
    request_body = CreateSessionOptions,
    responses((status = 200, body = User), (status = 400, body = Error)),
    security(("captcha" = []))
)]
async fn register_account(
    ValidatedJson(mut data): ValidatedJson<RegisterAccountOptions>,
) -> Result<Json<User>> {
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

#[utoipa::path(
    get,
    path = "/auth/accounts/verify/{id}/{code}",
    responses((status = 400, body = Error)),
    params(("id" = u64, path), ("code" = String, path))    
)]
pub async fn verify_account(Path((user_id, code)): Path<(u64, String)>) -> Result<()> {
    if email::verify(user_id, &code).await {
        let mut user = user_id.user().await?;
        user.verified = true;
        user.update().await;
        Ok(())
    } else {
        Err(Error::UnknownAccount)
    }
}

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};

    Router::new()
        .route(
            "/register",
            post(register_account).route_layer(middleware::from_fn(captcha::handle)),
        )
        .route("/verify/:user_id/:code", get(verify_account))
}
