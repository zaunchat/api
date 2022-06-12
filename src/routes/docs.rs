use axum::{extract::Json, routing::get, Router};
use utoipa::{Modify, OpenApi, openapi::security::{ApiKey, ApiKeyValue, SecurityScheme}};
use crate::structures::*;
use super::{
    auth::accounts::{self, *},
    auth::sessions::{self, *},
    bots,
    channels::{self, *},
    invites::{self, *},
    messages::{self, *},
    servers as server,
    servers::channels::*,
    servers::invites::*,
    servers::members::*,
    servers::roles::*,
    servers::servers::{self, *},
    users,
};
use crate::middlewares::ratelimit::RateLimitInfo;
use crate::utils::{Error, ValidationError, Badges, Permissions};

#[derive(OpenApi)]
#[openapi(
    handlers(
        accounts::register_account,
        accounts::verify_account,
        bots::create_bot,
        bots::delete_bot,
        bots::fetch_bot,
        bots::fetch_bots,
        channels::add_user_to_group,
        channels::create_group,
        channels::delete_group,
        channels::fetch_channel,
        channels::fetch_channels,
        channels::remove_user_from_group,
        invites::create_invite,
        invites::fetch_invite,
        invites::join_invite,
        messages::delete_message,
        messages::edit_message,
        messages::fetch_message,
        messages::send_message,
        server::channels::create_server_channel,
        server::channels::delete_server_channel,
        server::channels::edit_server_channel,
        server::invites::create_server_invite,
        server::invites::delete_server_invite,
        server::invites::fetch_server_invite,
        server::invites::fetch_server_invites,
        server::members::edit_member,
        server::members::fetch_member,
        server::members::fetch_members,
        server::members::kick_member,
        server::roles::create_role,
        server::roles::delete_role,
        server::roles::edit_role,
        server::roles::fetch_role,
        server::roles::fetch_roles,
        servers::create_server,
        servers::delete_server,
        servers::fetch_server,
        servers::fetch_servers,
        sessions::create_session,
        sessions::delete_session,
        sessions::fetch_session,
        sessions::fetch_sessions,
        users::fetch_me,
        users::fetch_user,
    ),
    components(
        Badges,
        Bot,
        Channel,
        ChannelTypes,
        Error,
        Invite,
        Member,
        Overwrite,
        OverwriteTypes,
        Permissions,
        Role,
        Server,
        Session,
        Message,
        User,
        RateLimitInfo,
        CreateMessageOptions,
        EditMessageOptions,
        CreateGroupOptions,
        CreateInviteOptions,
        CreateSessionOptions,
        RegisterAccountOptions,
        CreateServerChannelOptions,
        CreateServerInviteOptions,
        CreateServerOptions,
        CreateRoleOptions,
        UpdateRoleOptions,
        UpdateMemberOptions,
        ValidationError
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "users", description = "User Information"),
        (name = "messages", description = "Messaging"),
        (name = "accounts", description = "Accounts"),
        (name = "sessions", description = "Sessions"),
        (name = "channels", description = "Group/DM Channels"),
        (name = "servers", description = "Servers"),
        (name = "server::roles", description = "Server Roles"),
        (name = "server::members", description = "Server Members"),
        (name = "server::channels", description = "Server Channels"),
        (name = "server::invites", description = "Server Invites"),
        (name = "bots", description = "Bots"),
        (name = "invites", description = "Invites")
    )
)]
struct Docs;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "token",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("authorization"))),
            );

            components.add_security_scheme(
                "captcha",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("x-captcha-key"))),
            )
        }
    }
}


pub fn docs(router: Router) -> Router {
    let docs = Docs::openapi();
    router.route("/openapi.json", get(move || async { Json(docs) }))
}
