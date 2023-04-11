use crate::database::redis::*;
use crate::structures::*;
use crate::utils::error::Error;
use axum::{
    http::{header, Request},
    middleware::Next,
    response::Response,
};
use rmp_serde as MessagePack;

const ONE_HOUR_IN_SECONDS: i64 = 3600;

async fn fetch_from_cache(token: &str) -> Option<User> {
    REDIS
        .get(token)
        .await
        .ok()
        .and_then(|buf: Vec<u8>| MessagePack::from_slice(&buf).ok())
}

async fn cache_user(token: &str, user: &User) {
    let buf = MessagePack::to_vec_named(&user.with_hidden_fields()).unwrap();

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
}

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

    let Some(token) = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok()) else { Err(Error::MissingHeader)? };

    let user = match fetch_from_cache(token).await {
        Some(u) => u,
        _ => {
            let Some(u) = User::fetch_by_token(token).await else { Err(Error::InvalidToken)? };
            cache_user(token, &u).await;
            u
        }
    };

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
