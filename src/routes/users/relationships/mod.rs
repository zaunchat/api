pub mod add;
pub mod block;
pub mod delete;
pub mod fetch;

pub fn routes() -> axum::Router {
    use axum::{routing::*, Router};

    Router::new().route("/", get(fetch::fetch_many)).route(
        "/:user_id",
        delete(delete::delete).post(add::add).put(block::block),
    )
}
