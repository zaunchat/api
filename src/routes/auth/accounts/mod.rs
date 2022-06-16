pub mod login;
pub mod register;
pub mod verify;

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};

    Router::new()
        .route(
            "/register",
            post(register::register).route_layer(middleware::from_fn(captcha::handle)),
        )
        .route("/login", post(login::login))
        .route("/verify/:user_id/:code", get(verify::verify))
}
