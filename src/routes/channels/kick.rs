use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn kick(
    Extension(user): Extension<User>,
    Path((group_id, target_id)): Path<(u64, u64)>,
) -> Result<()> {
    let target = target_id.user().await?;
    let mut group = group_id.channel(user.id.into()).await?;

    if let Some(recipients) = group.recipients.as_mut() {
        let exists = recipients
            .iter()
            .position(|&id| id as u64 == target.id)
            .map(|i| recipients.remove(i))
            .is_some();

        if !exists {
            return Err(Error::UnknownMember);
        }
    }

    let permissions = Permissions::fetch(&user, None, group.id.into()).await?;

    permissions.has(Permissions::KICK_MEMBERS)?;

    group.update().await;

    Ok(())
}
