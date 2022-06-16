use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_one(
    Extension(user): Extension<User>,
    Path((server_id, channel_id)): Path<(u64, u64)>,
) -> Result<Json<Channel>> {
    user.member_of(server_id).await?;

    let channel = Channel::find_one(|q| q.eq("id", channel_id).eq("server_id", server_id)).await;

    match channel {
        Some(channel) => Ok(Json(channel)),
        None => Err(Error::UnknownChannel),
    }
}

pub async fn fetch_many(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
) -> Result<Json<Vec<Channel>>> {
    user.member_of(server_id).await?;

    let channels = Channel::find(|q| q.eq("server_id", server_id)).await;

    Ok(Json(channels))
}
