use super::auth::{accounts, sessions};
use super::{channels, invites, messages, servers};
use crate::structures::*;
use axum::{extract::Json, routing::get, Router};

pub fn document(app: Router) -> Router {
    let schema = describe_api! {
        info: {
            title: "ItChat API",
            version: "0.0.0",
        },
        tags: {
            users,
            servers,
            roles,
            messages,
            groups,
            channels,
            members,
            auth,
            invites,
            bots
        },
        servers: { "https://api.itchat.world" },
        security_schemes: {},
        paths: {
            // Accounts/Sessions
            ("auth/accounts/login"): {
                POST: { 200: Session, body: sessions::create::CreateSessionOptions, tags: {auth} } },
            ("auth/accounts/verify"): {
                GET: {
                    200: None,
                    tags: {auth},
                    parameters: {
                        (query user_id: u64): {},
                        (query code: String): {},
                    }
                }
            },
            ("auth/accounts/register"): {
                POST: { 200: User, body: accounts::register::RegisterAccountOptions, tags: {auth} }
            },
            ("auth/sessions"): {
                POST: { 200: Session, body: sessions::create::CreateSessionOptions, tags: {auth} }
            },
            ("auth/sessions" / { session_id: u64 }): {
                DELETE: { 200: None, tags: {auth} },
                GET: { 200: Session, tags: {auth} }
            },

            // Users
            ("users/@me"): { GET: { 200: User, tags: {users} } },
            ("users" / { user_id: u64 }): { GET: { 200: User, tags: {users} } },


            // Channels
            ("channels"): {
                GET: { 200: Vec<Channel>, tags:{channels} },
                POST: { 200("Create a group channel"): Channel, body: channels::create::CreateGroupOptions, tags:{channels} }
            },
            ("channels" / { channel_id: u64 }): {
                GET: { 200: Channel, tags:{channels} },
                DELETE: { 200: None, tags:{channels} },
                PATCH: { 200: Channel, body: channels::edit::EditGroupOptions, tags:{channels} }
            },
            ("channels" / { channel_id: u64 } / { user_id: u64 }): {
                DELETE: { 200: None, tags:{channels} }
            },

            // Messages
            ("channels" / { channel_id: u64 } / "messages"): {
                POST: { 200: Message, body: messages::create::CreateMessageOptions, tags:{messages} }
            },

            ("channels" / { channel_id: u64 } / "messages" / { message_id: u64 }): {
                GET: { 200: Message, tags:{messages} },
                PATCH: { 200: Message, body: messages::edit::EditMessageOptions, tags:{messages} },
                DELETE: { 200: None, tags:{messages} }
            },


            // Bots
            ("bots"): {
                POST: { 200: Bot, tags:{bots} },
                GET: { 200: Vec<Bot>, tags:{bots} }
            },
            ("bots" / { bot_id: u64 }): {
                GET: { 200: Bot, tags:{bots} },
                DELETE: { 200: None, tags:{bots} }
            },

            // Invites
            ("invites"): {
                POST: { 200: Invite, body: invites::create::CreateInviteOptions, tags:{invites} }
            },
            ("invites" / { code: String }): {
                GET: { 200: Invite, tags:{invites} },
                POST: { 200: Invite, tags:{invites} }
            },


            // Servers
            ("servers"): {
                POST: { 200: Server, body: servers::create::CreateServerOptions, tags:{servers} },
                GET: { 200: Vec<Server>, tags:{servers} }
            },

            ("servers" / { server_id: u64 }): {
                GET: { 200: Server, tags:{servers} },
                DELETE: { 200: None, tags:{servers} },
                PATCH: { 200: Server, body: servers::edit::EditServerOptions, tags:{servers} }
            },

            // Members
            ("servers" / { server_id: u64 } / "members"): {
                GET: { 200: Vec<Member>, tags:{members} },
                DELETE: { 200: Member, tags:{members} }
            },

            ("servers" / { server_id: u64 } / "members" / { user_id: u64 }): {
                GET: { 200: Member, tags:{members} },
                PATCH: { 200: Member, body: servers::members::edit::EditMemberOptions, tags:{members} }
            },

            // Roles
            ("servers" / { server_id: u64 } / "roles"): {
                POST: { 200: Role, body: servers::roles::create::CreateRoleOptions, tags:{roles} },
                GET: { 200: Vec<Role>, tags:{roles} }
            },

            ("servers" / { server_id: u64 } / "roles" / { role_id: u64 }): {
                GET: { 200: Role, tags:{roles} },
                DELETE: { 200: None, tags:{roles} },
                PATCH: { 200: Role, tags:{roles} }
            },

            // Server Invites
            ("servers" / { server_id: u64 } / "invites"): {
                GET: { 200: Vec<Invite>, tags:{invites} }
            },

            ("servers" / { server_id: u64 } / "invites" / { invite_id: u64 }): {
                GET: { 200: Invite, tags:{invites} },
                DELETE: { 200: None, tags:{invites} }
            },

            // Server Channels
            ("servers" / { server_id: u64 } / "channels"): {
                GET: { 200: Vec<Channel>, tags:{channels} },
                POST: { 200: Channel, body: servers::channels::create::CreateServerChannelOptions, tags:{channels} }
            },

            ("servers" / { server_id: u64 } / "channels" / { channel_id: u64 }): {
                GET: { 200: Channel, tags:{channels} },
                DELETE: { 200: None, tags:{channels} },
                PATCH: { 200: Channel, body: servers::channels::edit::EditServerChannelOptions, tags:{channels} }
            },
        }
    };

    app.route("/openapi.json", get(move || async { Json(schema) }))
}
