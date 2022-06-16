pub mod channels;
pub mod invites;
pub mod members;
pub mod roles;

pub mod create;
pub mod delete;
pub mod edit;
pub mod fetch;

pub fn routes() -> axum::Router {
    use axum::{routing::*, Router};

    Router::new()
        .route("/", post(create::create).get(fetch::fetch_many))
        .route(
            "/:server_id",
            get(fetch::fetch_one)
                .delete(delete::delete)
                .patch(edit::edit),
        )
        .nest("/:server_id/channels", channels::routes())
        .nest("/:server_id/invites", invites::routes())
        .nest("/:server_id/roles", roles::routes())
        .nest("/:server_id/members", members::routes())
}
