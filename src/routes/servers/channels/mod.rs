pub fn routes() -> axum::Router {
    use axum::{routing::*, Router};

    Router::new()
        .route("/", get(fetch_server_channels).post(create_server_channel))
        .route(
            "/:channel_id",
            get(fetch_server_channel)
                .patch(edit_server_channel)
                .delete(delete_server_channel),
        )
}