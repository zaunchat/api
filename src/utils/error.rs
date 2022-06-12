use crate::middlewares::ratelimit::RateLimitInfo;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug, Serialize)]
pub struct ValidationError(pub ValidationErrors);

#[derive(thiserror::Error, Debug, Serialize, utoipa::Component)]
#[serde(tag = "type")]
pub enum Error {
    #[error("Invalid body")]
    ValidationFailed { error: ValidationError },
    #[error("You have executed the rate limit")]
    RateLimited(RateLimitInfo),
    #[error("Invalid JSON")]
    ParseFailed,
    #[error("You need to verify your account in order to perform this action")]
    AccountVerificationRequired,
    #[error("Unauthorized. Provide a valid token and try again")]
    InvalidToken,
    #[error("Missing header")]
    MissingHeader,
    #[error("This email already in use")]
    EmailAlreadyInUse,
    #[error("This username taken by someone else")]
    UsernameTaken,
    #[error("Captcha don't love you")]
    FailedCaptcha,
    #[error("You lack permissions to perform that action")]
    MissingPermissions,
    #[error("You missing access to do the following action")]
    MissingAccess,
    #[error("Cannot send an empty message")]
    EmptyMessage,
    #[error("You must habe an invite to register")]
    RequireInviteCode,
    #[error("This invite already taken")]
    InviteAlreadyTaken,

    // Unknown
    #[error("Unknwon account")]
    UnknownAccount,
    #[error("Unknwon session")]
    UnknownSession,
    #[error("Unknwon user")]
    UnknownUser,
    #[error("Unknwon message")]
    UnknownMessage,
    #[error("Unknwon server")]
    UnknownServer,
    #[error("Unknwon member")]
    UnknownMember,
    #[error("Unknwon role")]
    UnknownRole,
    #[error("Unknwon bot")]
    UnknownBot,
    #[error("Unknwon channel")]
    UnknownChannel,
    #[error("Unknwon invite")]
    UnknownInvite,
    #[error("Unknwon error")]
    Unknown,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Error::RateLimited { .. } => StatusCode::TOO_MANY_REQUESTS,
            Error::InvalidToken => StatusCode::UNAUTHORIZED,
            _ => StatusCode::BAD_REQUEST,
        };
        (status, Json(serde_json::json!(self))).into_response()
    }
}

use utoipa::openapi::{schema::Component, ComponentType, PropertyBuilder};

impl utoipa::Component for ValidationError {
    fn component() -> Component {
        PropertyBuilder::new()
            .component_type(ComponentType::Object)
            .into()
    }
}
