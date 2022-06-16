#[utoipa::path(
    delete,
    path = "/servers/{server_id}/channels/{id}",
    responses((status = 400, body = Error)),
    params(("server_id" = u64, path), ("id" = u64, path))
)]
pub async fn delete_server_channel(
    Extension(user): Extension<User>,
    Path((server_id, channel_id)): Path<(u64, u64)>,
) -> Result<()> {
    user.member_of(server_id).await?;

    Permissions::fetch(&user, server_id.into(), None).await?.has(Permissions::MANAGE_CHANNELS)?;

    channel_id.channel(None).await?.delete().await;

    Ok(())
}
