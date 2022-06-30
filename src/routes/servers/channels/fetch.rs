use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_one(Path((server_id, channel_id)): Path<(u64, u64)>) -> Result<Json<Channel>> {
    let channel = channel_id.channel(None).await?;

    if channel.server_id != Some(server_id) {
        return Err(Error::UnknownChannel);
    }

    Ok(Json(channel))
}

pub async fn fetch_many(Path(server_id): Path<u64>) -> Json<Vec<Channel>> {
    Json(Channel::find(|q| q.eq("server_id", server_id)).await)
}
