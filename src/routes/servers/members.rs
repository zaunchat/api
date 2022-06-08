use crate::guards::r#ref::Ref;
use crate::structures::*;
use crate::utils::error::*;
use crate::utils::permissions::Permissions;
use rocket::serde::{json::Json, Deserialize};
use validator::Validate;

#[get("/<server_id>/<member_id>")]
async fn fetch_one(user: User, server_id: u64, member_id: Ref) -> Result<Json<Member>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let member = member_id.member(server_id).await?;

    Ok(Json(member))
}

#[get("/<server_id>?<limit>")]
async fn fetch_many(user: User, server_id: u64, limit: Option<u32>) -> Result<Json<Vec<Member>>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let mut limit = limit.unwrap_or(100);

    // The maximum limit is 1000
    if limit > 1000 {
        limit = 1000;
    }

    let members = Member::find(|q| q.eq("server_id", server_id).limit(limit.into())).await;

    Ok(Json(members))
}

#[delete("/<server_id>/<member_id>")]
async fn kick(user: User, server_id: u64, member_id: Ref) -> Result<()> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    if user.id != member_id.0 {
        let p = Permissions::fetch(&user, Some(server_id), None).await?;
        if !p.contains(Permissions::KICK_MEMBERS) {
            return Err(Error::MissingPermissions);
        }
    }

    member_id.member(server_id).await?.delete(member_id.0).await;

    Ok(())
}

#[derive(Deserialize, Validate)]
struct UpdateMemberSchema<'a> {
    #[validate(length(min = 1, max = 32))]
    nickname: Option<&'a str>,
    roles: Option<Vec<u64>>,
}

#[patch("/<server_id>/<member_id>", data = "<data>")]
async fn update(
    user: User,
    server_id: u64,
    member_id: Ref,
    data: Json<UpdateMemberSchema<'_>>,
) -> Result<Json<Member>> {
    let data = data.into_inner();

    data.validate()
        .map_err(|error| Error::InvalidBody { error })?;

    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let mut member = member_id.member(server_id).await?;
    let p = Permissions::fetch(&user, Some(server_id), None).await?;

    if let Some(nickname) = data.nickname {
        if !p.contains(Permissions::CHANGE_NICKNAME) && !p.contains(Permissions::MANAGE_NICKNAMES) {
            return Err(Error::MissingPermissions);
        }

        if nickname.len() == 0 {
            member.nickname = None;
        } else {
            member.nickname = Some(nickname.into())
        }
    }

    if let Some(ids) = data.roles {
        if !p.contains(Permissions::MANAGE_ROLES) {
            return Err(Error::MissingPermissions);
        }

        let mut roles = Role::find(|q| q.eq("server_id", server_id))
            .await
            .into_iter();

        member.roles = vec![];

        for id in ids {
            if roles.find(|r| r.id == id).is_none() {
                return Err(Error::UnknownRole);
            }
            member.roles.push(id);
        }
    }

    member.update().await;

    Ok(Json(member))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![fetch_one, fetch_many, update, kick]
}
