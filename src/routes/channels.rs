use crate::guards::r#ref::Ref;
use crate::structures::*;
use crate::utils::error::*;
use crate::utils::permissions::Permissions;
use rocket::form::validate::Contains;
use rocket::serde::{json::Json, Deserialize};
use validator::Validate;

#[get("/")]
async fn fetch_many(user: User) -> Json<Vec<Channel>> {
    Json(user.fetch_channels().await)
}

#[get("/<channel_id>")]
async fn fetch_one(user: User, channel_id: Ref) -> Result<Json<Channel>> {
    let channel = channel_id.channel(user.id.into()).await?;
    Ok(Json(channel))
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

#[put("/<group_id>/<target>")]
async fn add_user_to_group(user: User, group_id: Ref, target: Ref) -> Result<()> {
    let target = target.user().await?;
    let mut group = group_id.channel(user.id.into()).await?;

    if let Some(recipients) = group.recipients.as_mut() {
        if recipients.contains(&target.id) {
            return Err(Error::MissingAccess);
        }
        recipients.push(target.id);
    }

    group.update().await;

    Ok(())
}

#[delete("/<group_id>/<target>")]
async fn remove_user_from_group(user: User, group_id: Ref, target: Ref) -> Result<()> {
    let target = target.user().await?;
    let mut group = group_id.channel(user.id.into()).await?;

    if let Some(recipients) = group.recipients.as_mut() {
        if !recipients.contains(&target.id) {
            return Err(Error::UnknownMember);
        }

        // FIXME: Do the same thing in more efficient way

        let mut index: Option<usize> = None;

        for (i, id) in recipients.into_iter().enumerate() {
            if *id == target.id {
                index = Some(i);
                break;
            }
        }

        recipients.remove(index.unwrap());
    }

    let permissions = Permissions::fetch(&user, None, Some(group.id)).await?;

    if !permissions.contains(Permissions::KICK_MEMBERS) {
        return Err(Error::MissingPermissions);
    }

    group.update().await;

    Ok(())
}

#[delete("/<group_id>")]
async fn delete_group(user: User, group_id: Ref) -> Result<()> {
    let group = group_id.channel(user.id.into()).await?;

    if group.owner_id != Some(user.id) {
        return Err(Error::MissingPermissions);
    }

    group.delete(group.id).await;

    Ok(())
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        fetch_one,
        fetch_many,
        create_group,
        delete_group,
        add_user_to_group,
        remove_user_from_group,
    ]
}