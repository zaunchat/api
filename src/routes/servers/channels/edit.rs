#[utoipa::path(
    patch,
    path = "/servers/{server_id}/channels/{id}",
    responses((status = 200, body = Channel), (status = 400, body = Error)),
    params(("server_id" = u64, path), ("id" = u64, path))
)]
pub async fn edit_server_channel(
    Extension(user): Extension<User>,
    Path((server_id, channel_id)): Path<(u64, u64)>,
) -> Result<Json<Channel>> {
    user.member_of(server_id).await?;

    Permissions::fetch(&user, server_id.into(), channel_id.into()).await?.has(Permissions::MANAGE_CHANNELS)?;

    todo!("Update channels route")
}