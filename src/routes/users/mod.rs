mod fetch;

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};

    Router::new()
        .route("/@me", get(fetch::fetch_me))
        .route("/:user_id", get(fetch::fetch_one))
        .layer(middleware::from_fn(ratelimit::handle!(20, 1000 * 5)))
}
