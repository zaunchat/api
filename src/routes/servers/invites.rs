use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, utoipa::Component)]
pub struct CreateServerInviteOptions {
    channel_id: u64,
}

#[utoipa::path(
    get,
    path = "/servers/{server_id}/invites/{id}",
    responses((status = 200, body = Invite), (status = 400, body = Error)),
    params(("server_id" = u64, path), ("id" = u64, path))
)]
pub async fn fetch_server_invite(
    Extension(user): Extension<User>,
    Path((server_id, invite_id)): Path<(u64, u64)>,
) -> Result<Json<Invite>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    Ok(Json(invite_id.invite(server_id.into()).await?))
}

#[utoipa::path(
    get,
    path = "/servers/{server_id}/invites",
    responses((status = 200, body = [Invite]), (status = 400, body = Error)),
    params(("server_id" = u64, path))
)]
pub async fn fetch_server_invites(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
) -> Result<Json<Vec<Invite>>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let invites = Invite::find(|q| q.eq("server_id", server_id)).await;

    Ok(Json(invites))
}

#[utoipa::path(
    delete,
    path = "/servers/{server_id}/invites/{id}",
    responses((status = 400, body = Error)),
    params(("server_id" = u64, path), ("id" = u64, path))
)]
pub async fn delete_server_invite(
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

#[utoipa::path(
    post,
    path = "/servers/{server_id}/invites",
    responses((status = 200, body = Invite), (status = 400, body = Error)),
    params(("server_id" = u64, path))
)]
pub async fn create_server_invite(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
    ValidatedJson(data): ValidatedJson<CreateServerInviteOptions>,
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
        .route("/", get(fetch_server_invites).post(create_server_invite))
        .route(
            "/:invite_id",
            get(fetch_server_invite).delete(delete_server_invite),
        )
}
