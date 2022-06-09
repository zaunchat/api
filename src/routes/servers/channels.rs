use crate::guards::r#ref::Ref;
use crate::structures::*;
use crate::utils::error::*;
use crate::utils::permissions::Permissions;
use rocket::serde::{json::Json, Deserialize};
use validator::Validate;

#[derive(Deserialize, Validate, JsonSchema)]
struct CreateChannelSchema<'a> {
    r#type: ChannelTypes,
    #[validate(length(min = 1, max = 32))]
    name: &'a str,
}

#[openapi]
#[get("/<server_id>/<channel_id>")]
async fn fetch_one(user: User, server_id: u64, channel_id: u64) -> Result<Json<Channel>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let channel = Channel::find_one(|q| q.eq("id", channel_id).eq("server_id", server_id)).await;

    match channel {
        Some(channel) => Ok(Json(channel)),
        None => Err(Error::UnknownChannel),
    }
}

#[openapi]
#[get("/<server_id>")]
async fn fetch_many(user: User, server_id: u64) -> Result<Json<Vec<Channel>>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let channels = Channel::find(|q| q.eq("server_id", server_id)).await;

    Ok(Json(channels))
}

#[openapi]
#[post("/<server_id>", data = "<data>")]
async fn create(
    user: User,
    server_id: u64,
    data: Json<CreateChannelSchema<'_>>,
) -> Result<Json<Channel>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let p = Permissions::fetch(&user, server_id.into(), None).await?;

    if !p.contains(Permissions::MANAGE_CHANNELS) {
        return Err(Error::MissingPermissions);
    }

    match data.r#type {
        ChannelTypes::Text => Ok(Json(Channel::new_text(data.name.into(), server_id))),
        ChannelTypes::Category => Ok(Json(Channel::new_category(data.name.into(), server_id))),
        ChannelTypes::Voice => Ok(Json(Channel::new_voice(data.name.into(), server_id))),
        _ => Err(Error::MissingAccess),
    }
}

#[openapi]
#[delete("/<server_id>/<channel_id>")]
async fn delete(user: User, server_id: u64, channel_id: Ref) -> Result<()> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let p = Permissions::fetch(&user, server_id.into(), channel_id.0.into()).await?;

    if !p.contains(Permissions::MANAGE_CHANNELS) {
        return Err(Error::MissingPermissions);
    }

    channel_id.channel(None).await?.delete().await;

    Ok(())
}

#[openapi]
#[patch("/<server_id>/<channel_id>")]
async fn update(user: User, server_id: u64, channel_id: Ref) -> Result<Json<Channel>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let p = Permissions::fetch(&user, server_id.into(), channel_id.0.into()).await?;

    if !p.contains(Permissions::MANAGE_CHANNELS) {
        return Err(Error::MissingPermissions);
    }

    todo!("Update channels route")
}

#[openapi]
#[post("/<server_id>/<channel_id>/invite")]
async fn create_invite(user: User, server_id: u64, channel_id: Ref) -> Result<Json<Invite>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let channel = channel_id.channel(user.id.into()).await?;

    let p = Permissions::fetch(&user, server_id.into(), channel.id.into()).await?;

    if !p.contains(Permissions::INVITE_OTHERS) {
        return Err(Error::MissingPermissions);
    }

    let invite = Invite::new(user.id, channel.id, server_id.into());

    invite.save().await;

    Ok(Json(invite))
}

pub fn routes() -> (Vec<rocket::Route>, rocket_okapi::okapi::openapi3::OpenApi) {
    openapi_get_routes_spec![fetch_one, fetch_many, create, update, delete, create_invite]
}
