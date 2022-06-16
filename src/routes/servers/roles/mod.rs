pub fn routes() -> axum::Router {
    use axum::{routing::*, Router};

    Router::new()
        .route("/", get(fetch_roles).post(create_role))
        .route(
            "/:role_id",
            get(fetch_role).patch(edit_role).delete(delete_role),
        )
}