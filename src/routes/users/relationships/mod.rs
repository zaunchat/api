pub mod add;
pub mod block;
pub mod delete;

pub fn routes() -> axum::Router {
    use axum::{routing::*, Router};

    Router::new().route(
        "/:user_id",
        delete(delete::delete).post(add::add).put(block::block),
    )
}
