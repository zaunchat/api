pub mod accounts;
pub mod sessions;

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, Router};

    Router::new()
        .nest("/accounts", accounts::routes())
        .nest("/sessions", sessions::routes())
        .layer(middleware::from_fn(ratelimit::handle!(
            10,
            1000 * 60 * 60 * 3
        )))
}
