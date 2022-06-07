use rocket::{Build, Rocket};

mod auth;
mod bots;
mod invites;
mod messages;
mod servers;
mod users;
mod channels;

#[get("/")]
pub fn root() -> String {
    "Up".into()
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/", routes![root])
        .mount("/auth/accounts", auth::accounts::routes())
        .mount("/auth/sessions", auth::sessions::routes())
        .mount("/users", users::routes())
        .mount("/invites", invites::routes())
        .mount("/bots", bots::routes())
        .mount("/channels", channels::routes())
        .mount("/messages", messages::routes())
        .mount("/servers", servers::servers::routes())
        .mount("/servers/channels", servers::channels::routes())
        .mount("/servers/members", servers::members::routes())
        .mount("/servers/invites", servers::invites::routes())
}
