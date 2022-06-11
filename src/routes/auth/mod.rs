mod accounts;
mod sessions;

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, Router};

    Router::new()
        .nest("/accounts", accounts::routes())
        .nest("/sessions", sessions::routes())
        .layer(middleware::from_fn(ratelimit::handle!(
            2,
            1000 * 60 * 60 * 3
        )))
}
