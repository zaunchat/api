pub mod create;
pub mod delete;
pub mod edit;
pub mod fetch;

pub fn routes() -> axum::Router {
    use axum::{routing::*, Router};

    Router::new()
        .route("/", get(fetch::fetch_many).post(create::create))
        .route(
            "/:role_id",
            get(fetch::fetch_one)
                .patch(edit::edit)
                .delete(delete::delete),
        )
}
