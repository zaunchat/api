use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_one(Path((server_id, channel_id)): Path<(i64, i64)>) -> Result<Json<Channel>> {
    let channel = channel_id.channel(None).await?;

    if channel.server_id != Some(server_id) {
        return Err(Error::UnknownChannel);
    }

    Ok(Json(channel))
}

pub async fn fetch_many(Path(server_id): Path<i64>) -> Json<Vec<Channel>> {
    Json(
        Channel::select()
            .filter("server_id = $1")
            .bind(server_id)
            .fetch_all(pool())
            .await
            .unwrap(),
    )
}
