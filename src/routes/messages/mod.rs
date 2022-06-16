pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};

    Router::new()
        .route("/", post(send_message))
        .route("/:message_id", get(fetch_message).patch(edit_message).delete(delete_message))
        .layer(middleware::from_fn(ratelimit::handle!(10, 1000 * 10)))
}
