pub mod create;
pub mod delete;
pub mod edit;
pub mod fetch;

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};

    Router::new()
        .route("/", post(create::create))
        .route("/", get(fetch::fetch_many))
        .route(
            "/:message_id",
            get(fetch::fetch_one)
                .patch(edit::edit)
                .delete(delete::delete),
        )
        .layer(middleware::from_fn(ratelimit::handle!(10, 1000 * 10)))
}
