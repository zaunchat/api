pub mod create;
pub mod fetch;
pub mod join;

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};
    Router::new()
        .route("/", post(create::create))
        .route("/:code", get(fetch::fetch_one).post(join::join))
        .layer(middleware::from_fn(ratelimit::handle!(30, 1000 * 60 * 60)))
}
