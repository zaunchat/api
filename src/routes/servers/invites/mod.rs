pub mod delete;
pub mod fetch;

pub fn routes() -> axum::Router {
    use axum::{routing::*, Router};

    Router::new()
        .route("/", get(fetch::fetch_many))
        .route("/:invite_id", get(fetch::fetch_one).delete(delete::delete))
}
