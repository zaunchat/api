use rocket::{Build, Rocket};
use rocket_okapi::settings::OpenApiSettings;

mod auth;
mod bots;
mod channels;
mod invites;
mod messages;
mod servers;
mod users;

#[openapi]
#[get("/")]
pub fn root() -> String {
    "Up".into()
}

pub fn mount(mut rocket: Rocket<Build>) -> Rocket<Build> {
    let settings = OpenApiSettings::default();

    mount_endpoints_and_merged_docs! {
        rocket, "/".to_owned(), settings,
        "" => openapi_get_routes_spec![root],
        "/auth/accounts" => auth::accounts::routes(),
        "/auth/sessions" => auth::sessions::routes(),
        "/users" => users::routes(),
        "/@me/channels" => channels::routes(),
        "/messages" => messages::routes(),
        "/bots" => bots::routes(),
        "/invites" => invites::routes(),

        // Servers
        "/servers" => servers::servers::routes(),
        "/channels" => servers::channels::routes(),
        "/members" => servers::members::routes(),
        "/invites" => servers::invites::routes(),
        "/roles" => servers::roles::routes()
    };

    rocket
}
