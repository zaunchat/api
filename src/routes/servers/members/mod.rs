pub fn routes() -> axum::Router {
    use axum::{routing::*, Router};

    Router::new().route("/", get(fetch_members)).route(
        "/:member_id",
        get(fetch_member).patch(edit_member).delete(kick_member),
    )
}
