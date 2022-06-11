use axum::{
    http::{self, Request},
    middleware::Next,
    response::Response,
};
use crate::structures::*;
use crate::utils::error::Error;

pub async fn handle<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, Error> {
    let path = req.uri().path();

    let should_ignore = match path {
        "/" => true,
        "/auth/accounts/register" => true,
        "/auth/sessions/login" => true,
        _ if path.starts_with("/auth/accounts/verify") => true,
        _ => false,
    };

    if should_ignore {
        return Ok(next.run(req).await);
    }

    let token = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if token.is_none() {
        return Err(Error::MissingHeader);
    }

    let user = User::fetch_by_token(token.unwrap()).await;

    match user {
        Ok(user) => {
            req.extensions_mut().insert(user);
            Ok(next.run(req).await)
        }
        _ => Err(Error::InvalidToken),
    }
}
