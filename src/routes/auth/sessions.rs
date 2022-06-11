use crate::extractors::*;
use crate::structures::*;
use crate::utils::error::*;
use crate::utils::r#ref::Ref;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateSessionOptions {
    #[validate(length(min = 8, max = 32))]
    pub password: String,
    #[validate(email)]
    pub email: String,
}

async fn create(ValidatedJson(data): ValidatedJson<CreateSessionOptions>) -> Result<Json<Session>> {
    let user = User::find_one(|q| q.eq("email", &data.email)).await;

    match user {
        Some(user) => {
            if !user.verified {
                return Err(Error::AccountVerificationRequired);
            }

            if argon2::verify_encoded(user.password.as_str(), data.password.to_string().as_bytes())
                .is_err()
            {
                return Err(Error::MissingAccess);
            }

            let session = Session::new(user.id);

            session.save().await;

            Ok(Json(session))
        }
        _ => Err(Error::UnknownAccount),
    }
}

async fn fetch_one(Extension(user): Extension<User>, Path(id): Path<u64>) -> Result<Json<Session>> {
    Ok(Json(id.session(user.id).await?))
}

pub async fn fetch_many(Extension(user): Extension<User>) -> Json<Vec<Session>> {
    Json(Session::find(|q| q.eq("user_id", &user.id)).await)
}

pub async fn delete_session(
    Extension(user): Extension<User>,
    Path((id, token)): Path<(u64, String)>,
) -> Result<()> {
    let session = id.session(user.id).await?;

    if session.token != token {
        return Err(Error::InvalidToken);
    }

    session.delete().await;

    Ok(())
}

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};

    Router::new()
        .route("/", get(fetch_many))
        .route(
            "/login",
            post(create).route_layer(middleware::from_fn(captcha::handle)),
        )
        .route("/:session_id", get(fetch_one))
        .route("/:session_id/:token", delete(delete_session))
}
