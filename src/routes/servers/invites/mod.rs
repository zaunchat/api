pub fn routes() -> axum::Router {
    use axum::{routing::*, Router};

    Router::new()
        .route("/", get(fetch_server_invites).post(create_server_invite))
        .route(
            "/:invite_id",
            get(fetch_server_invite).delete(delete_server_invite),
        )
}
