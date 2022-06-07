use std::io::Cursor;

use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::ValidationErrors;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Error {
    InvalidBody {
        #[serde(skip_serializing, skip_deserializing)]
        error: ValidationErrors,
    },
    InvalidToken,
    Unauthorized,
    EmailAlreadyInUse,
    NotVerified,
    UsernameTaken,
    FailedCaptcha,
    MissingPermissions,
    MissingAccess,

    // Unknown
    UnknownAccount,
    UnknownSession,
    UnknownUser,
    UnknownMessage,
    UnknownServer,
    UnknownMember,
    UnknownRole,
    UnknownBot,
    UnknownChannel,
    UnknownInvite,
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let body = json!(self).to_string();
        // TODO: Get status
        let status = Status::Forbidden;

        Response::build()
            .sized_body(body.len(), Cursor::new(body))
            .header(ContentType::JSON)
            .status(status)
            .ok()
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
pub type Success = Result<()>;
