use crate::extractors::*;
use crate::utils::*;
use crate::structures::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, utoipa::Component)]
pub struct CreateMessageOptions {
    channel_id: u64,
    #[validate(length(min = 1, max = 2000))]
    content: String,
}

#[derive(Deserialize, Validate, utoipa::Component)]
pub struct EditMessageOptions {
    #[validate(length(min = 1, max = 2000))]
    content: String,
}

#[utoipa::path(
    post,
    request_body = CreateMessageOptions,
    path = "/messages",
    responses((status = 200, body = Message), (status = 400, body = Error))
)]
pub async fn send_message(
    Extension(user): Extension<User>,
    ValidatedJson(data): ValidatedJson<CreateMessageOptions>,
) -> Result<Json<Message>> {
    let p = Permissions::fetch(&user, None, data.channel_id.into()).await?;

    if !p.contains(Permissions::VIEW_CHANNEL) || !p.contains(Permissions::SEND_MESSAGES) {
        return Err(Error::MissingPermissions);
    }

    let mut msg = Message::new(data.channel_id, user.id);

    // TODO: Add more fields
    msg.content = data.content.into();

    if msg.is_empty() {
        return Err(Error::EmptyMessage);
    }

    msg.save().await;

    Ok(Json(msg))
}

#[utoipa::path(
    patch,
    request_body = EditMessageOptions,
    path = "/messages/{id}",
    responses((status = 200, body = Message), (status = 400, body = Error)),
    params(("id" = u64, path))
)]
pub async fn edit_message(
    Extension(user): Extension<User>,
    Path(message_id): Path<u64>,
    ValidatedJson(data): ValidatedJson<EditMessageOptions>,
) -> Result<Json<Message>> {
    let mut msg = message_id.message().await?;

    if msg.author_id != user.id {
        return Err(Error::MissingPermissions);
    }

    let p = Permissions::fetch(&user, None, msg.channel_id.into()).await?;

    if !p.contains(Permissions::VIEW_CHANNEL) {
        return Err(Error::MissingPermissions);
    }

    msg.content = data.content.into();
    msg.update().await;

    Ok(Json(msg))
}

#[utoipa::path(
    delete,
    path = "/messages/{id}",
    responses((status = 200), (status = 400, body = Error)),
    params(("id" = u64, path))
)]
pub async fn delete_message(
    Extension(user): Extension<User>,
    Path(message_id): Path<u64>,
) -> Result<()> {
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

#[utoipa::path(
    get,
    path = "/messages/{id}",
    responses((status = 200, body = Message), (status = 400, body = Error)),
    params(("id" = u64, path))
)]
pub async fn fetch_message(
    Extension(user): Extension<User>,
    Path(message_id): Path<u64>,
) -> Result<Json<Message>> {
    let msg = message_id.message().await?;
    let p = Permissions::fetch(&user, None, msg.channel_id.into()).await?;

    if !p.contains(Permissions::VIEW_CHANNEL) || !p.contains(Permissions::READ_MESSAGE_HISTORY) {
        return Err(Error::MissingPermissions);
    }

    Ok(Json(msg))
}

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};

    Router::new()
        .route("/", post(send_message))
        .route(
            "/:message_id",
            get(fetch_message)
                .patch(edit_message)
                .delete(delete_message),
        )
        .layer(middleware::from_fn(ratelimit::handle!(10, 1000 * 10)))
}
