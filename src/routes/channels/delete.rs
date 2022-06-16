use crate::config::*;
use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[utoipa::path(
    delete,
    path = "/channels/{id}",
    responses((status = 200), (status = 400, body = Error)),
    params(("id" = u64, path))
)]
pub async fn delete(
    Extension(user): Extension<User>,
    Path(channel_id): Path<u64>,
) -> Result<()> {
    let channel = channel_id.channel(user.id.into()).await?;

    if channel.owner_id != Some(user.id) {
        return Err(Error::MissingPermissions);
    }

    channel.delete().await;

    Ok(())
}