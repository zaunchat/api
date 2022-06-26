use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct EditMemberOptions {
    #[validate(length(min = 1, max = 32))]
    nickname: Option<String>,
    roles: Option<Vec<u64>>,
}

pub async fn edit(
    Extension(user): Extension<User>,
    Path((server_id, member_id)): Path<(u64, u64)>,
    ValidatedJson(data): ValidatedJson<EditMemberOptions>,
) -> Result<Json<Member>> {
    user.member_of(server_id).await?;

    let mut member = member_id.member(server_id).await?;
    let p = Permissions::fetch(&user, server_id.into(), None).await?;

    if let Some(nickname) = &data.nickname {
        p.has(Permissions::CHANGE_NICKNAME)?;
        p.has(Permissions::MANAGE_NICKNAMES)?;
        member.nickname = if nickname.is_empty() {
            None
        } else {
            Some(nickname.into())
        };
    }

    if let Some(ids) = &data.roles {
        p.has(Permissions::MANAGE_ROLES)?;

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

    publish(server_id, Payload::ServerMemberUpdate(member.clone())).await;

    Ok(Json(member))
}
