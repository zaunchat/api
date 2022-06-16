pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};

    Router::new()
        .route("/", get(fetch_channels).post(create_group))
        .route("/:channel_id", get(fetch_channel).delete(delete_group))
        .route("/:channel_id/:target/kick", delete(remove_user_from_group))
        .layer(middleware::from_fn(ratelimit::handle!(15, 1000 * 5)))
}
