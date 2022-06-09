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
        "/channels" => channels::routes(),
        "/messages" => messages::routes(),
        "/bots" => bots::routes(),
        "/invites" => invites::routes(),
        "/servers" => servers::servers::routes(),
        "/servers/channels" => servers::channels::routes(),
        "/servers/members" => servers::members::routes(),
        "/servers/invites" => servers::invites::routes(),
        "/servers/roles" => servers::roles::routes()
    };

    rocket
}
