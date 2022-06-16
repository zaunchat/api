pub mod create;
pub mod delete;
pub mod fetch;

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};

    Router::new()
        .route("/", get(fetch::fetch_many).post(create::create))
        .route("/:bot_id", get(fetch::fetch_one).delete(delete::delete))
        .layer(middleware::from_fn(ratelimit::handle!(20, 1000 * 5)))
}
