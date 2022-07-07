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

pub async fn create(
    ValidatedJson(data): ValidatedJson<CreateSessionOptions>,
) -> Result<Json<Session>> {
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

            let session = Session::new(user.id);

            Ok(Json(session.insert(pool()).await.unwrap()))
        }
        _ => Err(Error::UnknownAccount),
    }
}
