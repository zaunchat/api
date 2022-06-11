use crate::extractors::*;
use crate::utils::error::*;
use crate::utils::permissions::Permissions;
use crate::{structures::*, utils::r#ref::Ref};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
struct CreateGroupSchema {
    #[validate(length(min = 3, max = 32))]
    name: String,
}

async fn fetch_many(Extension(user): Extension<User>) -> Json<Vec<Channel>> {
    Json(user.fetch_channels().await)
}

async fn fetch_one(
    Extension(user): Extension<User>,
    Path(channel_id): Path<u64>,
) -> Result<Json<Channel>> {
    let channel = channel_id.channel(user.id.into()).await?;
    Ok(Json(channel))
}

async fn create_group(
    Extension(user): Extension<User>,
    ValidatedJson(data): ValidatedJson<CreateGroupSchema>,
) -> Result<Json<Channel>> {
    let group = Channel::new_group(user.id, data.name);

    group.save().await;

    Ok(Json(group))
}

async fn add_user_to_group(
    Extension(user): Extension<User>,
    Path((group_id, target_id)): Path<(u64, u64)>,
) -> Result<()> {
    let target = target_id.user().await?;
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

async fn remove_user_from_group(
    Extension(user): Extension<User>,
    Path((group_id, target_id)): Path<(u64, u64)>,
) -> Result<()> {
    let target = target_id.user().await?;
    let mut group = group_id.channel(user.id.into()).await?;

    if let Some(recipients) = group.recipients.as_mut() {
        let exists = recipients
            .iter()
            .position(|&id| id == target.id)
            .map(|i| recipients.remove(i))
            .is_some();

        if !exists {
            return Err(Error::UnknownMember);
        }
    }

    let permissions = Permissions::fetch(&user, None, group.id.into()).await?;

    if !permissions.contains(Permissions::KICK_MEMBERS) {
        return Err(Error::MissingPermissions);
    }

    group.update().await;

    Ok(())
}

async fn delete_group(Extension(user): Extension<User>, Path(channel_id): Path<u64>) -> Result<()> {
    let group = channel_id.channel(user.id.into()).await?;

    if group.owner_id != Some(user.id) {
        return Err(Error::MissingPermissions);
    }

    group.delete().await;

    Ok(())
}

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};

    Router::new()
        .route("/", get(fetch_many).post(create_group))
        .route("/:channel_id", get(fetch_one).delete(delete_group))
        .route(
            "/join/:group_id/:target",
            post(add_user_to_group).delete(remove_user_from_group),
        )
        .route("/leave/:group_id/:target", delete(remove_user_from_group))
        .layer(middleware::from_fn(ratelimit::handle!(15, 1000 * 5)))

}
