use crate::structures::{Base, Channel, Invite, Member, User};
use crate::utils::error::*;
use rocket::serde::json::Json;

#[get("/<code>")]
async fn fetch_invite(code: &str) -> Result<Json<Invite>> {
    let invite = Invite::find_one(|q| q.eq("code", &code)).await;

    if let Some(invite) = invite {
        return Ok(Json(invite));
    }

    Err(Error::UnknownInvite)
}

#[post("/<code>")]
async fn join_invite(user: User, code: &str) -> Result<()> {
    let invite = Invite::find_one(|q| q.eq("code", &code)).await;

    match invite {
        Some(mut invite) if invite.server_id.is_some() => {
            let already_joined =
                Member::find_one(|q| q.eq("id", &user.id).eq("server_id", &invite.server_id)).await;

            if already_joined.is_some() {
                return Err(Error::MissingAccess);
            }

            let member = Member::new(user.id, invite.server_id.unwrap());

            invite.uses = invite.uses + 1;
            member.save().await;
            invite.save().await;

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

            group.save().await;

            Ok(())
        }
        None => Err(Error::UnknownInvite),
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![fetch_invite, join_invite]
}
