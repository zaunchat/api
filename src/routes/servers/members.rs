use crate::extractors::*;
use crate::utils::error::*;
use crate::utils::permissions::Permissions;
use crate::{structures::*, utils::r#ref::Ref};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, utoipa::Component)]
pub struct UpdateMemberOptions {
    #[validate(length(min = 1, max = 32))]
    nickname: Option<String>,
    roles: Option<Vec<u64>>,
}

#[derive(Deserialize, Validate, utoipa::Component)]
pub struct FetchMembersOptions {
    #[validate(range(min = 2, max = 1000))]
    limit: Option<u64>,
}

#[utoipa::path(
    get,
    path = "/servers/{server_id}/members/{user_id}",
    responses((status = 200, body = Member), (status = 400, body = Error)),
    params(("server_id" = u64, path), ("user_id" = u64, path))
)]
pub async fn fetch_member(
    Extension(user): Extension<User>,
    Path((server_id, member_id)): Path<(u64, u64)>,
) -> Result<Json<Member>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let member = member_id.member(server_id).await?;

    Ok(Json(member))
}

#[utoipa::path(
    get,
    path = "/servers/{server_id}/members",
    responses((status = 200, body = [Member]), (status = 400, body = Error)),
    params(("server_id" = u64, path))
)]
pub async fn fetch_members(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
    Query(query): Query<FetchMembersOptions>,
) -> Result<Json<Vec<Member>>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let limit = query.limit.unwrap_or(100);
    let members = Member::find(|q| q.eq("server_id", server_id).limit(limit)).await;

    Ok(Json(members))
}

#[utoipa::path(
    delete,
    path = "/servers/{server_id}/members/{user_id}",
    responses((status = 400, body = Error)),
    params(("server_id" = u64, path), ("user_id" = u64, path))
)]
pub async fn kick_member(
    Extension(user): Extension<User>,
    Path((server_id, member_id)): Path<(u64, u64)>,
) -> Result<()> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    if user.id != member_id {
        let p = Permissions::fetch(&user, Some(server_id), None).await?;
        if !p.contains(Permissions::KICK_MEMBERS) {
            return Err(Error::MissingPermissions);
        }
    }

    member_id.member(server_id).await?.delete().await;

    Ok(())
}

#[utoipa::path(
    patch,
    request_body = UpdateMemberOptions,
    path = "/servers/{server_id}/members/{user_id}",
    responses((status = 200, body = Member), (status = 400, body = Error)),
    params(("server_id" = u64, path), ("user_id" = u64, path))
)]
pub async fn edit_member(
    Extension(user): Extension<User>,
    Path((server_id, member_id)): Path<(u64, u64)>,
    ValidatedJson(data): ValidatedJson<UpdateMemberOptions>,
) -> Result<Json<Member>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let mut member = member_id.member(server_id).await?;
    let p = Permissions::fetch(&user, Some(server_id), None).await?;

    if let Some(nickname) = &data.nickname {
        if !p.contains(Permissions::CHANGE_NICKNAME) && !p.contains(Permissions::MANAGE_NICKNAMES) {
            return Err(Error::MissingPermissions);
        }

        if nickname.is_empty() {
            member.nickname = None;
        } else {
            member.nickname = Some(nickname.into())
        }
    }

    if let Some(ids) = &data.roles {
        if !p.contains(Permissions::MANAGE_ROLES) {
            return Err(Error::MissingPermissions);
        }

        let mut roles = Role::find(|q| q.eq("server_id", server_id))
            .await
            .into_iter();

        member.roles = vec![];

        for &id in ids {
            if !roles.any(|r| r.id == id) {
                return Err(Error::UnknownRole);
            }
            member.roles.push(id);
        }
    }

    member.update().await;

    Ok(Json(member))
}

pub fn routes() -> axum::Router {
    use axum::{routing::*, Router};

    Router::new().route("/", get(fetch_members)).route(
        "/:member_id",
        get(fetch_member).patch(edit_member).delete(kick_member),
    )
}
