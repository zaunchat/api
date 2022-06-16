pub mod create;
pub mod delete;
pub mod fetch;

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};

    Router::new()
        .route(
            "/",
            get(fetch::fetch_many)
                .post(create::create)
                .route_layer(middleware::from_fn(captcha::handle)),
        )
        .route("/:session_id", get(fetch::fetch_one).delete(delete::delete))
}
