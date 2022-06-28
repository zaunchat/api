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

    if token.is_none() {
        return Err(Error::MissingHeader);
    }

    let user = User::fetch_by_token(token.unwrap()).await;

    match user {
        Some(user) => {
            req.extensions_mut().insert(user);
            Ok(next.run(req).await)
        }
        _ => Err(Error::InvalidToken),
    }
}
