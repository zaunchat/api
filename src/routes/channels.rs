use crate::database::DB as db;
use crate::structures::{Base, Channel, User};
use crate::utils::error::*;
use crate::utils::permissions::Permissions;
use rocket::form::validate::Contains;
use rocket::serde::{json::Json, Deserialize};
use validator::Validate;

#[get("/")]
async fn fetch_channels(user: User) -> Json<Vec<Channel>> {
    let channels: Vec<Channel> = db
        .fetch(
            "SELECT * FROM channels WHERE recipients::jsonb ? $1",
            vec![user.id.into()],
        )
        .await
        .unwrap();
    Json(channels)
}

#[get("/<channel_id>")]
async fn fetch_channel(user: User, channel_id: u64) -> Result<Json<Channel>> {
    let channel: Option<Channel> = db
        .fetch(
            "SELECT * FROM channels WHERE recipients::jsonb ? $1 AND id = $2",
            vec![user.id.into(), channel_id.into()],
        )
        .await
        .unwrap();

    if let Some(channel) = channel {
        return Ok(Json(channel));
    } else {
        return Err(Error::UnknownChannel);
    }
}

#[derive(Debug, Deserialize, Validate, Clone, Copy)]
struct CreateGroupSchema<'a> {
    #[validate(length(min = 3, max = 32))]
    name: &'a str,
}

#[post("/", data = "<data>")]
async fn create_group(user: User, data: Json<CreateGroupSchema<'_>>) -> Result<Json<Channel>> {
    let data = data.into_inner();

    data.validate()
        .map_err(|error| Error::InvalidBody { error })?;

    let group = Channel::new_group(user.id, data.name.into());

    group.save().await;

    Ok(Json(group))
}

#[put("/<group_id>/<user_id>")]
async fn add_user_to_group(user: User, group_id: u64, user_id: u64) -> Result<()> {
    let channel: Option<Channel> = db
        .fetch(
            "SELECT * FROM channels WHERE recipients::jsonb ? $1 AND id = $2",
            vec![user.id.into(), group_id.into()],
        )
        .await
        .unwrap();

    let target = User::find_one_by_id(user_id).await;

    match (channel, target) {
        (Some(mut channel), Some(target)) => {
            if let Some(recipients) = channel.recipients.as_mut() {
                if recipients.contains(&target.id) {
                    return Err(Error::MissingAccess);
                }
                recipients.push(target.id);
            }

            channel.update().await;

            Ok(())
        }
        (None, Some(_)) => Err(Error::UnknownChannel),
        (Some(_), None) => Err(Error::UnknownUser),
        _ => Err(Error::MissingAccess),
    }
}

#[delete("/<group_id>/<user_id>")]
async fn remove_user_to_group(user: User, group_id: u64, user_id: u64) -> Result<()> {
    let channel: Option<Channel> = db
        .fetch(
            "SELECT * FROM channels WHERE recipients::jsonb ? $1 AND id = $2",
            vec![user.id.into(), group_id.into()],
        )
        .await
        .unwrap();

    let target = User::find_one_by_id(user_id).await;

    match (channel, target) {
        (Some(mut channel), Some(target)) => {
            if let Some(recipients) = channel.recipients.as_mut() {
                if !recipients.contains(&target.id) {
                    return Err(Error::UnknownMember);
                }
                // FIXME: Do the same thing in more efficient way

                let mut index: Option<usize> = None;

                for (i, id) in recipients.into_iter().enumerate() {
                    if *id == target.id {
                        index = Some(i);
                        break
                    }
                }

                recipients.remove(index.unwrap());
            }

            let permissions = Permissions::fetch(user, None, Some(channel.id)).await;

            if !permissions.contains(Permissions::KICK_MEMBERS) {
                return Err(Error::MissingPermissions);
            }

            channel.update().await;

            Ok(())
        }
        (None, Some(_)) => Err(Error::UnknownChannel),
        (Some(_), None) => Err(Error::UnknownUser),
        _ => Err(Error::MissingAccess),
    }
}


#[delete("/<group_id>")]
async fn delete_group(user: User, group_id: u64) -> Result<()> {
    let channel: Option<Channel> = db
        .fetch(
            "SELECT * FROM channels WHERE recipients::jsonb ? $1 AND id = $2",
            vec![user.id.into(), group_id.into()],
        )
        .await
        .unwrap();

    match channel {
        Some(channel) if channel.owner_id == Some(user.id) => {
            channel.delete(channel.id).await;
            Ok(())
        },
        Some(_) => Err(Error::MissingPermissions),
        _ => Err(Error::UnknownChannel)
    }

}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        fetch_channels,
        fetch_channel,
        create_group,
        add_user_to_group,
        remove_user_to_group,
        delete_group
    ]
}
