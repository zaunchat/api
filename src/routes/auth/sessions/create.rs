use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct CreateSessionOptions {
    #[validate(length(min = 8, max = 32))]
    pub password: String,
    #[validate(email)]
    pub email: String,
}

pub async fn create(ValidatedJson(data): ValidatedJson<CreateSessionOptions>) -> Result<String> {
    let user = User::select()
        .filter("email = $1")
        .bind(data.email)
        .fetch_one(pool())
        .await;

    match user {
        Ok(user) => {
            if !user.verified {
                return Err(Error::AccountVerificationRequired);
            }

            if argon2::verify_encoded(user.password.as_str(), data.password.to_string().as_bytes())
                .is_err()
            {
                return Err(Error::MissingAccess);
            }

            let session = Session::new(user.id).save().await?;

            Ok(session.token)
        }
        _ => Err(Error::UnknownAccount),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;

    #[test]
    fn execute() {
        run(async {
            let user = User::faker().save().await.unwrap();

            let payload = CreateSessionOptions {
                email: user.email.clone(),
                password: "passw0rd".to_string(),
            };

            create(ValidatedJson(payload)).await.unwrap();

            user.remove().await.unwrap();
        })
    }
}
