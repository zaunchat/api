use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn edit(
    Extension(user): Extension<User>,
    Path((server_id, channel_id)): Path<(u64, u64)>,
) -> Result<Json<Channel>> {
    user.member_of(server_id).await?;

    Permissions::fetch(&user, server_id.into(), channel_id.into())
        .await?
        .has(Permissions::MANAGE_CHANNELS)?;

    todo!("Update channels route")
}
