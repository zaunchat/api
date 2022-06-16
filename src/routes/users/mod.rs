pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};

    Router::new()
        .route("/@me", get(fetch_me))
        .route("/:user_id", get(fetch_user))
        .layer(middleware::from_fn(ratelimit::handle!(20, 1000 * 5)))
}
