pub mod edit;
pub mod fetch;
pub mod kick;

pub fn routes() -> axum::Router {
    use axum::{routing::*, Router};

    Router::new().route("/", get(fetch::fetch_many)).route(
        "/:member_id",
        get(fetch::fetch_one).patch(edit::edit).delete(kick::kick),
    )
}
