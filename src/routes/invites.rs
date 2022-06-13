use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, utoipa::Component)]
pub struct CreateInviteOptions {
    channel_id: u64,
}

#[utoipa::path(
    get,
    path = "/invites/{code}",
    responses((status = 200, body = Invite), (status = 400, body = Error)),
    params(("code" = String, path))
)]
pub async fn fetch_invite(Path(code): Path<String>) -> Result<Json<Invite>> {
    let invite = Invite::find_one(|q| q.eq("code", &code)).await;

    if let Some(invite) = invite {
        return Ok(Json(invite));
    }

    Err(Error::UnknownInvite)
}

#[utoipa::path(
    post,
    path = "/invites/{code}",
    responses((status = 400, body = Error)),
    params(("code" = String, path))
)]
pub async fn join_invite(Extension(user): Extension<User>, Path(code): Path<String>) -> Result<()> {
    let invite = Invite::find_one(|q| q.eq("code", &code)).await;

    match invite {
        Some(mut invite) if invite.server_id.is_some() => {
            if user.is_in_server(invite.server_id.unwrap()).await {
                return Err(Error::MissingAccess);
            }

            let member = Member::new(user.id, invite.server_id.unwrap());

            invite.uses += 1;
            member.save().await;
            invite.update().await;

            Ok(())
        }
        Some(invite) => {
            let mut group = Channel::find_one_by_id(invite.channel_id).await.unwrap();

            if let Some(recipients) = group.recipients.as_mut() {
                if recipients.contains(&user.id) {
                    return Err(Error::MissingAccess);
                }
                recipients.push(user.id);
            } else {
                unreachable!()
            }

            group.update().await;

            Ok(())
        }
        None => Err(Error::UnknownInvite),
    }
}

#[utoipa::path(
    post,
    path = "/invites",
    request_body = CreateInviteOptions,
    responses((status = 200, body = Invite), (status = 400, body = Error))
)]
pub async fn create_invite(
    Extension(user): Extension<User>,
    ValidatedJson(data): ValidatedJson<CreateInviteOptions>,
) -> Result<Json<Invite>> {
    let channel = data.channel_id.channel(user.id.into()).await?;

    let p = Permissions::fetch(&user, channel.server_id, channel.id.into()).await?;

    if !p.contains(Permissions::INVITE_OTHERS) {
        return Err(Error::MissingPermissions);
    }

    let invite = Invite::new(user.id, channel.id, channel.server_id);
    invite.save().await;

    Ok(Json(invite))
}

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};
    Router::new()
        .route("/", post(create_invite))
        .route("/:code", get(fetch_invite).post(join_invite))
        .layer(middleware::from_fn(ratelimit::handle!(30, 1000 * 60 * 60)))
}
