pub mod delete;
pub mod fetch;

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};

    Router::new()
        .route("/", get(fetch::fetch_many))
        .route("/:invite_id", get(fetch::fetch_one).delete(delete::delete))
        .layer(middleware::from_fn(member::handle))
}
