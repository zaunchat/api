use crate::extractors::*;
use crate::utils::error::*;
use crate::utils::permissions::Permissions;
use crate::{structures::*, utils::r#ref::Ref};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
struct CreateChannelOptions {
    r#type: ChannelTypes,
    #[validate(length(min = 1, max = 32))]
    name: String,
}

async fn fetch_one(
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

async fn fetch_many(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
) -> Result<Json<Vec<Channel>>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let channels = Channel::find(|q| q.eq("server_id", server_id)).await;

    Ok(Json(channels))
}

async fn create(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
    ValidatedJson(data): ValidatedJson<CreateChannelOptions>,
) -> Result<Json<Channel>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let p = Permissions::fetch(&user, server_id.into(), None).await?;

    if !p.contains(Permissions::MANAGE_CHANNELS) {
        return Err(Error::MissingPermissions);
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

async fn delete_channel(
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

async fn update(
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
        .route("/", get(fetch_many).post(create))
        .route(
            "/:channel_id",
            get(fetch_one).patch(update).delete(delete_channel),
        )
}
