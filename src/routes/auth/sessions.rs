use crate::extractors::*;
use crate::structures::*;
use crate::utils::error::*;
use crate::utils::r#ref::Ref;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, utoipa::Component)]
pub struct CreateSessionOptions {
    #[validate(length(min = 8, max = 32))]
    pub password: String,
    #[validate(email)]
    pub email: String,
}

#[utoipa::path(
    post,
    path = "/auth/sessions",
    request_body = CreateSessionOptions,
    responses((status = 200, body = Session), (status = 400, body = Error))
)]
async fn create_session(
    ValidatedJson(data): ValidatedJson<CreateSessionOptions>,
) -> Result<Json<Session>> {
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

#[utoipa::path(
    get,
    path = "/auth/sessions/{id}",
    responses((status = 200, body = Session), (status = 400, body = Error)),
    params(("id" = u64, path))
)]
async fn fetch_session(
    Extension(user): Extension<User>,
    Path(id): Path<u64>,
) -> Result<Json<Session>> {
    Ok(Json(id.session(user.id).await?))
}

#[utoipa::path(
    get,
    path = "/auth/sessions",
    responses((status = 200, body = [Session]), (status = 400, body = Error)),
    params(("id" = u64, path))
)]
pub async fn fetch_sessions(Extension(user): Extension<User>) -> Json<Vec<Session>> {
    Json(Session::find(|q| q.eq("user_id", &user.id)).await)
}

#[utoipa::path(
    delete,
    path = "/auth/sessions/{id}",
    responses((status = 400, body = Error)),
    params(("id" = u64, path))
)]
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
        .route("/", get(fetch_sessions))
        .route(
            "/login",
            post(create_session).route_layer(middleware::from_fn(captcha::handle)),
        )
        .route("/:session_id", get(fetch_session))
        .route("/:session_id/:token", delete(delete_session))
}
