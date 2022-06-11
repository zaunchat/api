use crate::extractors::*;
use crate::utils::error::*;
use crate::utils::permissions::Permissions;
use crate::{structures::*, utils::r#ref::Ref};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
struct CreateInviteOptions {
    channel_id: u64,
}

async fn fetch_one(
    Extension(user): Extension<User>,
    Path((server_id, invite_id)): Path<(u64, u64)>,
) -> Result<Json<Invite>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    Ok(Json(invite_id.invite(server_id.into()).await?))
}

async fn fetch_many(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
) -> Result<Json<Vec<Invite>>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let invites = Invite::find(|q| q.eq("server_id", server_id)).await;

    Ok(Json(invites))
}

async fn delete_invite(
    Extension(user): Extension<User>,
    Path((server_id, invite_id)): Path<(u64, u64)>,
) -> Result<()> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let p = Permissions::fetch(&user, server_id.into(), None).await?;

    if !p.contains(Permissions::MANAGE_INVITES) {
        return Err(Error::MissingPermissions);
    }

    invite_id.invite(server_id.into()).await?.delete().await;

    Ok(())
}

async fn create(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
    ValidatedJson(data): ValidatedJson<CreateInviteOptions>,
) -> Result<Json<Invite>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let channel = data.channel_id.channel(None).await?;
    let p = Permissions::fetch(&user, channel.server_id, channel.id.into()).await?;

    if !p.contains(Permissions::INVITE_OTHERS) {
        return Err(Error::MissingPermissions);
    }

    let invite = Invite::new(user.id, channel.id, channel.server_id);
    invite.save().await;

    Ok(Json(invite))
}

pub fn routes() -> axum::Router {
    use axum::{routing::*, Router};

    Router::new()
        .route("/", get(fetch_many).post(create))
        .route("/:invite_id", get(fetch_one).delete(delete_invite))
}
