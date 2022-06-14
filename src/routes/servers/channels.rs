use crate::config::*;
use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, utoipa::Component)]
pub struct CreateServerChannelOptions {
    r#type: ChannelTypes,
    #[validate(length(min = 1, max = 32))]
    name: String,
}

#[utoipa::path(
    get,
    path = "/servers/{server_id}/channels/{id}",
    responses((status = 200, body = Channel), (status = 400, body = Error)),
    params(("server_id" = u64, path), ("id" = u64, path))
)]
pub async fn fetch_server_channel(
    Extension(user): Extension<User>,
    Path((server_id, channel_id)): Path<(u64, u64)>,
) -> Result<Json<Channel>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let channel = Channel::find_one(|q| q.eq("id", channel_id).eq("server_id", server_id)).await;

    match channel {
        Some(channel) => Ok(Json(channel)),
        None => Err(Error::UnknownChannel),
    }
}

#[utoipa::path(
    get,
    path = "/servers/{server_id}/channels",
    responses((status = 200, body = [Channel]), (status = 400, body = Error)),
    params(("server_id" = u64, path))
)]
pub async fn fetch_server_channels(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
) -> Result<Json<Vec<Channel>>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let channels = Channel::find(|q| q.eq("server_id", server_id)).await;

    Ok(Json(channels))
}

#[utoipa::path(
    post,
    request_body = CreateServerChannelOptions,
    path = "/servers/{server_id}/channels",
    responses((status = 200, body = Channel), (status = 400, body = Error)),
    params(("server_id" = u64, path))
)]
pub async fn create_server_channel(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
    ValidatedJson(data): ValidatedJson<CreateServerChannelOptions>,
) -> Result<Json<Channel>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let p = Permissions::fetch(&user, server_id.into(), None).await?;

    if !p.contains(Permissions::MANAGE_CHANNELS) {
        return Err(Error::MissingPermissions);
    }

    let count = Channel::count(|q| q.eq("server_id", server_id)).await;

    if count > *MAX_SERVER_CHANNELS {
        return Err(Error::MaximumChannels);
    }

    let channel = match data.r#type {
        ChannelTypes::Text => Ok(Json(Channel::new_text(data.name.clone(), server_id))),
        ChannelTypes::Category => Ok(Json(Channel::new_category(data.name.clone(), server_id))),
        ChannelTypes::Voice => Ok(Json(Channel::new_voice(data.name.clone(), server_id))),
        _ => Err(Error::MissingAccess),
    };

    if let Ok(channel) = &channel {
        channel.save().await;
    }

    channel
}

#[utoipa::path(
    delete,
    path = "/servers/{server_id}/channels/{id}",
    responses((status = 400, body = Error)),
    params(("server_id" = u64, path), ("id" = u64, path))
)]
pub async fn delete_server_channel(
    Extension(user): Extension<User>,
    Path((server_id, channel_id)): Path<(u64, u64)>,
) -> Result<()> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let p = Permissions::fetch(&user, server_id.into(), channel_id.into()).await?;

    if !p.contains(Permissions::MANAGE_CHANNELS) {
        return Err(Error::MissingPermissions);
    }

    channel_id.channel(None).await?.delete().await;

    Ok(())
}

#[utoipa::path(
    patch,
    path = "/servers/{server_id}/channels/{id}",
    responses((status = 200, body = Channel), (status = 400, body = Error)),
    params(("server_id" = u64, path), ("id" = u64, path))
)]
pub async fn edit_server_channel(
    Extension(user): Extension<User>,
    Path((server_id, channel_id)): Path<(u64, u64)>,
) -> Result<Json<Channel>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let p = Permissions::fetch(&user, server_id.into(), channel_id.into()).await?;

    if !p.contains(Permissions::MANAGE_CHANNELS) {
        return Err(Error::MissingPermissions);
    }

    todo!("Update channels route")
}

pub fn routes() -> axum::Router {
    use axum::{routing::*, Router};

    Router::new()
        .route("/", get(fetch_server_channels).post(create_server_channel))
        .route(
            "/:channel_id",
            get(fetch_server_channel)
                .patch(edit_server_channel)
                .delete(delete_server_channel),
        )
}
