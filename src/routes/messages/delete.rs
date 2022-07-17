use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(
    Extension(user): Extension<User>,
    Path((channel_id, id)): Path<(i64, i64)>,
) -> Result<()> {
    let msg = id.message().await?;
    let p = Permissions::fetch(&user, None, channel_id.into()).await?;

    if msg.author_id != user.id {
        p.has(&[Permissions::MANAGE_MESSAGES])?;
    } else {
        p.has(&[Permissions::MANAGE_MESSAGES, Permissions::VIEW_CHANNEL])?;
    }

    let attachment_ids: Vec<i64> = msg
        .attachments
        .0
        .clone()
        .into_iter()
        .map(|a| a.id)
        .collect();

    let mut tx = pool().begin().await?;

    sqlx::query("UPDATE attachments SET deleted = TRUE WHERE id = ANY($1)")
        .bind(attachment_ids)
        .execute(&mut tx)
        .await?;

    msg.delete(&mut tx).await?;

    tx.commit().await?;

    publish(channel_id, Payload::MessageDelete(id.into())).await;

    Ok(())
}
