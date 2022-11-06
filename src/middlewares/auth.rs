use crate::structures::*;
use crate::utils::error::Error;
use axum::{
    http::{header, Request},
    middleware::Next,
    response::Response,
};
use rmp_serde as MessagePack;

use crate::database::redis::*;

const ONE_HOUR_IN_SECONDS: i64 = 3600;

pub async fn handle<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, Error> {
    let should_ignore = matches!(
        req.uri().path(),
        "/" | "/auth/accounts/register"
            | "/auth/accounts/login"
            | "/auth/accounts/verify"
            | "/auth/sessions"
            | "/openapi.json"
            | "/ws"
    );

    if should_ignore {
        return Ok(next.run(req).await);
    }

    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token = match token {
        Some(s) => s,
        _ => return Err(Error::MissingHeader),
    };

    let user = if let Ok(Ok(user)) = REDIS
        .get(token)
        .await
        .map(|buf: Vec<u8>| MessagePack::from_slice(&buf))
    {
        Some(user)
    } else {
        match User::fetch_by_token(token).await {
            Some(mut u) => {
                u.show_private_fields();

                let buf = MessagePack::to_vec_named(&u).unwrap();

                u.hide_private_fields();

                REDIS
                    .set::<(), _, _>(
                        token,
                        buf.as_slice(),
                        Expiration::EX(ONE_HOUR_IN_SECONDS).into(),
                        None,
                        false,
                    )
                    .await
                    .ok();

                Some(u)
            }
            _ => None,
        }
    };

    if let Some(user) = user {
        req.extensions_mut().insert(user);
        return Ok(next.run(req).await);
    }

    Err(Error::InvalidToken)
}
