use super::auth::{accounts, sessions};
use super::{channels, messages};
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
                POST: { 200: accounts::register::RegisterResponse, body: accounts::register::RegisterAccountOptions, tags: {auth} }
            },
            ("auth/sessions"): {
                POST: { 200: String, body: sessions::create::CreateSessionOptions, tags: {auth} }
            },
            ("auth/sessions" / { session_id: u64 }): {
                DELETE: { 200: None, tags: {auth} },
                GET: { 200: Session, tags: {auth} }
            },

            // Users
            ("users"): { GET: { 200: Vec<User>, tags: {users}} },
            ("users/@me"): { GET: { 200: User, tags: {users} } },
            ("users" / { user_id: u64 }): { GET: { 200: User, tags: {users} } },
            ("users" / { user_id: u64 } / "dm"): { GET: { 200: Channel, tags: {users} } },
            ("users/@me/relationships" / { user_id: u64 }): {
              POST: { 200: None, tags: {users} },
              PUT: { 200: None, tags: {users} },
              DELETE: { 200: None, tags: {users} }
            },

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
            ("messages" / { channel_id: u64 }): {
                POST: { 200: Message, body: messages::create::CreateMessageOptions, tags:{messages} }
            },

            ("messages" / { channel_id: u64 } / { message_id: u64 }): {
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
            }
        }
    };

    app.route("/openapi.json", get(move || async { Json(schema) }))
}
