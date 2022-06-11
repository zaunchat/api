use super::{
    auth::accounts::*, auth::sessions::*, bots::*, channels::*, invites::*, messages::*, users::*,
};
use crate::middlewares::ratelimit::RateLimitInfo;
use crate::structures::*;
use crate::utils::{badges::Badges, error::Error, permissions::Permissions};
use std::sync::Arc;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::Config;

#[derive(OpenApi)]
#[openapi(
    handlers(
        fetch_me,
        fetch_user,

        join_invite,
        create_invite,
        fetch_invite,

        fetch_channel,
        fetch_channels,
        delete_group,
        create_group,
        add_user_to_group,
        remove_user_from_group,

        fetch_message,
        send_message,
        edit_message,
        delete_message,

        fetch_bots,
        fetch_bot,
        delete_bot,
        create_bot,


        create_session,
        delete_session,
        fetch_session,
        fetch_sessions,

        register_account,
        verify_account
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
        RegisterAccountOptions
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "users", description = "Users")
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

use axum::{
    extract::Json, extract::*, http::StatusCode, response::IntoResponse, routing::get, Router,
};

pub fn docs(router: Router) -> Router {
    let config = Arc::new(Config::from("/openapi.json"));
    let docs = Docs::openapi();
    router
        .route("/openapi.json", get({ move || async { Json(docs) } }))
        .route(
            "/swagger/*tail",
            get(serve_swagger_ui).layer(Extension(config)),
        )
}

async fn serve_swagger_ui(
    Path(tail): Path<String>,
    Extension(state): Extension<Arc<Config<'static>>>,
) -> impl IntoResponse {
    match utoipa_swagger_ui::serve(&tail[1..], state) {
        Ok(file) => file
            .map(|file| {
                (
                    StatusCode::OK,
                    [("Content-Type", file.content_type)],
                    file.bytes,
                )
                    .into_response()
            })
            .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response()),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
