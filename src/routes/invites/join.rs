use crate::config::*;
use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn join(Extension(user): Extension<User>, Path(code): Path<String>) -> Result<()> {
    let invite = Invite::find_one(code).await;

    match invite {
        Ok(invite) if invite.server_id.is_some() => {
            let server_id = invite.server_id.unwrap();

            let already_joined = Member::select()
                .filter("id = $1 AND server_id = $2")
                .bind(user.id)
                .bind(server_id)
                .fetch_one(pool())
                .await
                .is_ok();

            if already_joined {
                return Err(Error::MissingAccess);
            }

            let count = Member::count(&format!("server_id = {}", server_id)).await;

            if count > *MAX_SERVER_MEMBERS {
                return Err(Error::MaximumChannels);
            }

            let member = Member::new(user.id, server_id).save().await?;

            invite
                .update_partial()
                .uses(invite.uses + 1)
                .update(pool())
                .await?;

            publish(
                user.id,
                Payload::ServerCreate(server_id.server(None).await?),
            )
            .await;

            publish(server_id, Payload::ServerMemberJoin(member)).await;

            Ok(())
        }
        Ok(invite) => {
            let mut group = Channel::find_one(invite.channel_id).await?;

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

            let group = group.update_all_fields(pool()).await?;

            publish(group.id, Payload::ChannelUpdate(group.clone())).await;
            publish(user.id, Payload::ChannelCreate(group)).await;

            Ok(())
        }
        _ => Err(Error::UnknownInvite),
    }
}
