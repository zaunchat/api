use crate::config::*;
use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn join(Extension(user): Extension<User>, Path(code): Path<String>) -> Result<()> {
    let invite = Invite::find_one(|q| q.eq("code", &code)).await;

    match invite {
        Some(mut invite) if invite.server_id.is_some() => {
            let server_id = invite.server_id.unwrap();
            let already_joined =
                Member::count(|q| q.eq("server_id", &server_id).eq("id", &user.id)).await;

            if already_joined != 0 {
                return Err(Error::MissingAccess);
            }

            let count = Member::count(|q| q.eq("server_id", server_id)).await;

            if count > *MAX_SERVER_MEMBERS {
                return Err(Error::MaximumChannels);
            }

            let member = Member::new(user.id, server_id);

            invite.uses += 1;
            member.save().await;
            invite.update().await;

            publish(
                user.id,
                Payload::ServerCreate(server_id.server(None).await?),
            )
            .await;
            publish(server_id, Payload::ServerMemberJoin(member)).await;

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

            publish(group.id, Payload::ChannelUpdate(group.clone())).await;
            publish(user.id, Payload::ChannelCreate(group)).await;

            Ok(())
        }
        None => Err(Error::UnknownInvite),
    }
}
