#[utoipa::path(
    get,
    path = "/servers/{server_id}/channels/{id}",
    responses((status = 200, body = Channel), (status = 400, body = Error)),
    params(("server_id" = u64, path), ("id" = u64, path))
)]
pub async fn fetch_server_channel(
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

#[utoipa::path(
    get,
    path = "/servers/{server_id}/channels",
    responses((status = 200, body = [Channel]), (status = 400, body = Error)),
    params(("server_id" = u64, path))
)]
pub async fn fetch_server_channels(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
) -> Result<Json<Vec<Channel>>> {
    user.member_of(server_id).await?;

    let channels = Channel::find(|q| q.eq("server_id", server_id)).await;

    Ok(Json(channels))
}