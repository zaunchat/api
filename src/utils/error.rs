use crate::middlewares::ratelimit::RateLimitInfo;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use quick_error::quick_error;
use serde::Serialize;
use std::fmt::Debug;

quick_error! {
    #[derive(Debug, Serialize, OpgModel)]
    pub enum Error {
        RateLimited(info: RateLimitInfo) {
            from(RateLimitInfo)
            display("Executed the rate limit. Please retry after {}s", info.retry_after)
        }
        InvalidBody { display("You have provided a bad json schema") }
        MissingHeader { display("Missing header") }
        AccountVerificationRequired { display("You need to verify your account in order to perform this action") }
        InvalidToken { display("Unauthorized. Provide a valid token and try again") }
        EmailAlreadyInUse { display("This email already in use") }
        MissingPermissions { display("You lack permissions to perform that action") }
        EmptyMessage { display("Cannot send an empty message") }
        RequireInviteCode { display("You must have an invite code to perform this action") }
        InviteAlreadyTaken { display("This invite already used") }
        FailedCaptcha { display("Respect the captcha, Respect you") }
        MissingAccess { display("You missing access to perform this action ") }
        DatabaseError { display("Database cannot process this operation") }


        Blocked
        BlockedByOther
        AlreadyFriends
        AlreadySendRequest

        UnknownAccount
        UnknownBot
        UnknownChannel
        UnknownInvite
        UnknownUser
        UnknownMessage
        UnknownServer
        UnknownSession
        UnknownRole
        UnknownMember
        Unknown { display("Unknown error has occurred") }

        MaximumFriends { display("Maximum number of friends reached") }
        MaximumServers { display("Maximum number of servers reached")  }
        MaximumGroups { display("Maximum number of groups reached")  }
        MaximumRoles { display("Maximum number of server roles reached")  }
        MaximumChannels { display("Maximum number of channels reached") }
        MaximumGroupMembers { display("Maximum number of group members reached") }
        MaximumBots { display("Maximum number of bots reached") }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        log::error!("Database Error: {}", err);
        Self::DatabaseError
    }
}

impl From<ormlite::Error> for Error {
    fn from(err: ormlite::Error) -> Self {
        log::error!("Database Error: {}", err);
        Self::DatabaseError
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Error::RateLimited { .. } => StatusCode::TOO_MANY_REQUESTS,
            Error::InvalidToken => StatusCode::UNAUTHORIZED,
            _ => StatusCode::BAD_REQUEST,
        };

        let mut body = serde_json::json!({ "type": self });
        let msg = self.to_string();

        if msg.contains(' ') {
            body["message"] = serde_json::json!(msg);
        }

        (status, Json(body)).into_response()
    }
}
