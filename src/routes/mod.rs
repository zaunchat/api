mod auth;
mod bots;
mod channels;
mod docs;
mod messages;
mod users;

async fn root() -> &'static str {
    "Up!"
}

use axum::{routing::*, Router};

pub fn mount(app: Router) -> Router {
    docs::document(app)
        .route("/", get(root))
        .nest("/auth", auth::routes())
        .nest("/users", users::routes())
        .nest("/bots", bots::routes())
        .nest("/channels", channels::routes())
        .nest("/messages/:channel_id", messages::routes())
}
