pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};
    Router::new()
        .route("/", post(create_invite))
        .route("/:code", get(fetch_invite).post(join_invite))
        .layer(middleware::from_fn(ratelimit::handle!(30, 1000 * 60 * 60)))
}
