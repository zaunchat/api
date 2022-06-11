use axum::{routing::*, Router};

mod auth;
mod bots;
mod channels;
mod docs;
mod invites;
mod messages;
mod servers;
mod users;

async fn root() -> &'static str {
    "Up!"
}

pub fn mount(app: Router) -> Router {
    docs::docs(app)
        .route("/", get(root))
        .nest("/auth", auth::routes())
        .nest("/users", users::routes())
        .nest("/invites", invites::routes())
        .nest("/bots", bots::routes())
        .nest("/messages", messages::routes())
        .nest("/channels", channels::routes())
        .nest("/servers", servers::routes())
}
