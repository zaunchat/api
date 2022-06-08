use crate::guards::captcha::Captcha;
use crate::guards::r#ref::Ref;
use crate::structures::{Base, Session, User};
use crate::utils::error::*;
use rocket::serde::{json::Json, Deserialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Clone, Copy)]
pub struct LoginSchema<'r> {
    #[validate(length(min = 8, max = 32))]
    pub password: &'r str,
    #[validate(email)]
    pub email: &'r str,
}

#[post("/login", data = "<data>")]
async fn create(_captcha: Captcha, data: Json<LoginSchema<'_>>) -> Result<Json<Session>> {
    let data = data.into_inner();

    data.validate()
        .map_err(|error| Error::InvalidBody { error })?;

    let user = User::find_one(|q| q.eq("email", &data.email)).await;

    match user {
        Some(user) => {
            if !user.verified {
                return Err(Error::NotVerified);
            }

            if argon2::verify_encoded(user.password.as_str(), data.password.to_string().as_bytes())
                .is_err()
            {
                return Err(Error::Unauthorized);
            }

            let session = Session::new(user.id);

            session.save().await;

            Ok(Json(session))
        }
        _ => Err(Error::UnknownAccount),
    }
}

#[get("/<target>")]
async fn fetch_one(user: User, target: Ref) -> Result<Json<Session>> {
    let session = target.session(user.id).await?;
    Ok(Json(session))
}

#[get("/")]
pub async fn fetch_many(user: User) -> Json<Vec<Session>> {
    Json(Session::find(|q| q.eq("user_id", &user.id)).await)
}

#[delete("/<target>/<token>")]
pub async fn delete(user: User, target: Ref, token: &str) -> Result<()> {
    let session = target.session(user.id).await?;

    if session.token != token {
        return Err(Error::InvalidToken);
    }

    session.delete(session.id).await;

    Ok(())
}

pub fn routes() -> Vec<rocket::Route> {
    // TODO: Add route to edit sessions
    routes![create, delete, fetch_one, fetch_many]
}
