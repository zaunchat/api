pub mod edit;
pub mod fetch;
pub mod kick;

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};

    Router::new()
        .route("/", get(fetch::fetch_many))
        .route(
            "/:id",
            get(fetch::fetch_one).patch(edit::edit).delete(kick::kick),
        )
        .layer(middleware::from_fn(member::handle))
}
