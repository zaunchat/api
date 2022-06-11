use crate::extractors::*;
use crate::utils::error::*;
use crate::utils::permissions::Permissions;
use crate::{structures::*, utils::r#ref::Ref};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
struct UpdateMemberOptions {
    #[validate(length(min = 1, max = 32))]
    nickname: Option<String>,
    roles: Option<Vec<u64>>,
}

#[derive(Deserialize, Validate)]
struct FetchMembersOptions {
    #[validate(range(min = 2, max = 1000))]
    limit: Option<u64>,
}

async fn fetch_one(
    Extension(user): Extension<User>,
    Path((server_id, member_id)): Path<(u64, u64)>,
) -> Result<Json<Member>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let member = member_id.member(server_id).await?;

    Ok(Json(member))
}

async fn fetch_many(
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

async fn kick(
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

async fn update(
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

    Router::new()
        .route("/", get(fetch_many))
        .route("/:member_id", get(fetch_one).patch(update).delete(kick))
}
