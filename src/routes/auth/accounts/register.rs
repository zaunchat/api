use crate::config::*;
use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use argon2::Config;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct RegisterAccountOptions {
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String,
    #[validate(email)]
    pub email: String,
    pub invite_code: Option<String>,
}

#[derive(Serialize, OpgModel)]
pub struct RegisterResponse {
    pub pending_verification: bool,
}

pub async fn register(
    ValidatedJson(mut data): ValidatedJson<RegisterAccountOptions>,
) -> Result<Json<RegisterResponse>> {
    data.email = email::normalize(data.email).expect("Non normalized email");

    let invite = if *REQUIRE_INVITE_TO_REGISTER && data.invite_code.is_some() {
        email::AccountInvite::get_one(data.invite_code.as_ref().unwrap(), pool())
            .await
            .ok()
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

    if let Some(invite) = invite {
        invite
            .update_partial()
            .taken_by(Some(user.id))
            .used(true)
            .update(pool())
            .await?;
    }

    let user = user.save().await?;

    Ok(RegisterResponse {
        pending_verification: !user.verified,
    }
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;

    #[test]
    fn execute() -> Result<(), Error> {
        run(async {
            let email = format!("test.{}@example.com", nanoid::nanoid!(6));
            let payload = RegisterAccountOptions {
                username: "test".to_string(),
                email: email.clone(),
                password: "passw0rd".to_string(),
                invite_code: None,
            };

            register(ValidatedJson(payload)).await?;

            let user = User::select()
                .filter("email = $1")
                .bind(email::normalize(email))
                .fetch_one(pool())
                .await?;

            user.remove().await?;

            Ok(())
        })
    }
}
