use crate::guards::r#ref::Ref;
use crate::structures::*;
use crate::utils::error::*;
use crate::utils::permissions::*;
use rocket::serde::{json::Json, Deserialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
struct CreateMessageSchema<'r> {
    channel_id: u64,
    #[validate(length(min = 1, max = 2000))]
    content: &'r str,
}

#[post("/", data = "<data>")]
async fn send(user: User, data: Json<CreateMessageSchema<'_>>) -> Result<Json<Message>> {
    let data = data.into_inner();

    data.validate()
        .map_err(|error| Error::InvalidBody { error })?;

    let p = Permissions::fetch(&user, None, data.channel_id.into()).await?;

    if !p.contains(Permissions::VIEW_CHANNEL) || !p.contains(Permissions::SEND_MESSAGES) {
        return Err(Error::MissingPermissions);
    }

    let mut msg = Message::new(data.channel_id, user.id);

    // TODO: Add more fields
    msg.content = Some(data.content.into());

    if msg.is_empty() {
        return Err(Error::EmptyMessage);
    }

    msg.save().await;

    Ok(Json(msg))
}

#[derive(Deserialize, Validate)]
struct EditMessageSchema<'r> {
    #[validate(length(min = 1, max = 2000))]
    content: &'r str,
}

#[patch("/<message_id>", data = "<data>")]
async fn edit(
    user: User,
    message_id: Ref,
    data: Json<EditMessageSchema<'_>>,
) -> Result<Json<Message>> {
    let data = data.into_inner();

    data.validate()
        .map_err(|error| Error::InvalidBody { error })?;

    let mut msg = message_id.message().await?;

    if msg.author_id != user.id {
        return Err(Error::MissingPermissions);
    }

    let p = Permissions::fetch(&user, None, msg.channel_id.into()).await?;

    if !p.contains(Permissions::VIEW_CHANNEL) {
        return Err(Error::MissingPermissions);
    }

    msg.content = Some(data.content.into());
    msg.update().await;

    Ok(Json(msg))
}

#[delete("/<message_id>")]
async fn delete(user: User, message_id: Ref) -> Result<()> {
    let msg = message_id.message().await?;
    let p = Permissions::fetch(&user, None, Some(msg.channel_id)).await?;

    if !p.contains(Permissions::VIEW_CHANNEL) {
        return Err(Error::MissingPermissions);
    }

    if msg.author_id != user.id && !p.contains(Permissions::MANAGE_MESSAGES) {
        return Err(Error::MissingPermissions);
    }

    msg.delete().await;

    Ok(())
}

#[get("/<message_id>")]
async fn fetch_one(user: User, message_id: Ref) -> Result<Json<Message>> {
    let msg = message_id.message().await?;
    let p = Permissions::fetch(&user, None, msg.channel_id.into()).await?;

    if !p.contains(Permissions::VIEW_CHANNEL) || !p.contains(Permissions::READ_MESSAGE_HISTORY) {
        return Err(Error::MissingPermissions);
    }

    Ok(Json(msg))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![send, edit, delete, fetch_one]
}
