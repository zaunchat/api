use crate::structures::{Base, Session, User};
use crate::utils::error::{Error, Result};
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
pub async fn login(data: Json<LoginSchema<'_>>) -> Result<Json<Session>> {
    let data = data.into_inner();

    data.validate()
        .map_err(|error| Error::InvalidBody { error })?;

    let user = User::find_one(|q| q.eq("email", &data.email)).await;

    if let Some(user) = user {
        if user.verified == false {
            return Err(Error::NotVerified);
        }

        if argon2::verify_encoded(user.password.as_str(), data.password.to_string().as_bytes())
            .is_err()
        {
            return Err(Error::Unauthorized);
        }

        let session = Session::new(user.id);

        session.save().await;

        return Ok(Json(session));
    } else {
        return Err(Error::AccountNotFound);
    }
}

#[get("/<session_id>")]
pub async fn fetch_session(user: User, session_id: i64) -> Result<Json<Session>> {
    let session = Session::find_one(|q| q.eq("user_id", &user.id).eq("id", &session_id)).await;

    if let Some(session) = session {
        Ok(Json(session))
    } else {
        Err(Error::SessionNotFound)
    }
}

#[get("/")]
pub async fn fetch_sessions(user: User) -> Json<Vec<Session>> {
    Json(Session::find(|q| q.eq("user_id", &user.id)).await)
}

#[delete("/<session_id>/<token>")]
pub async fn delete_session(user: User, session_id: i64, token: &str) -> Result<()> {
    let session = Session::find_one(|q| {
        q.eq("user_id", &user.id)
            .eq("id", &session_id)
            .eq("token", &token)
    })
    .await;

    if let Some(session) = session {
        session.delete(session.id).await;
        Ok(())
    } else {
        Err(Error::SessionNotFound)
    }
}
