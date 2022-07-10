pub mod fetch;
pub mod open_dm;
pub mod relationships;

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};

    Router::new()
        .nest("/@me/relationships", relationships::routes())
        .route("/", get(fetch::fetch_many))
        .route("/@me", get(fetch::fetch_me))
        .route("/:user_id", get(fetch::fetch_one))
        .route("/:user_id/dm", get(open_dm::open_dm))
        .layer(middleware::from_fn(ratelimit::handle!(20, 1000 * 5)))
}
