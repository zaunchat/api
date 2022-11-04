use crate::structures::*;
use crate::utils::error::Error;
use axum::{
    http::{header, Request},
    middleware::Next,
    response::Response,
};

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

    match User::fetch_by_token(token).await {
        Some(user) => {
            req.extensions_mut().insert(user);
            Ok(next.run(req).await)
        }
        _ => Err(Error::InvalidToken),
    }
}
