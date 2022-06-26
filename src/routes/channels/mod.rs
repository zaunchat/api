pub mod create;
pub mod delete;
pub mod fetch;
pub mod kick;

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};

    Router::new()
        .route("/", get(fetch::fetch_many).post(create::create))
        .route("/:channel_id", get(fetch::fetch_one).delete(delete::delete))
        .route("/:channel_id/:target", delete(kick::kick))
        .layer(middleware::from_fn(ratelimit::handle!(35, 1000 * 5)))
}
