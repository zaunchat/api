use crate::config::*;
use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;


#[utoipa::path(
    post,
    path = "/invites/{code}",
    responses((status = 400, body = Error)),
    params(("code" = String, path))
)]
pub async fn join(Extension(user): Extension<User>, Path(code): Path<String>) -> Result<()> {
    let invite = Invite::find_one(|q| q.eq("code", &code)).await;

    match invite {
        Some(mut invite) if invite.server_id.is_some() => {
            if user.member_of(invite.server_id.unwrap()).await.is_ok() {
                return Err(Error::MissingAccess);
            }

            let count = Member::count(|q| q.eq("server_id", invite.server_id.unwrap())).await;

            if count > *MAX_SERVER_MEMBERS {
                return Err(Error::MaximumChannels);
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
                if recipients.len() as u64 > *MAX_GROUP_MEMBERS {
                    return Err(Error::MaximumGroupMembers);
                }

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