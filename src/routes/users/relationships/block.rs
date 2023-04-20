use crate::config::MAX_BLOCKED;
use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn block(Extension(mut user): Extension<User>, Path(id): Path<Snowflake>) -> Result<()> {
    if *MAX_BLOCKED <= user.relations.len() as u64 {
        return Err(Error::MaximumBlocked);
    }

    if Some(&RelationshipStatus::Blocked) == user.relations.0.get(&id) {
        return Ok(());
    }

    let status = if Some(&RelationshipStatus::BlockedByOther) == user.relations.0.get(&id) {
        // The target blocked me, block him is well
        (RelationshipStatus::Blocked, RelationshipStatus::Blocked)
    } else {
        // Block the target
        (
            RelationshipStatus::Blocked,
            RelationshipStatus::BlockedByOther,
        )
    };

    let mut target = id.user().await?;

    user.relations.0.insert(target.id, status.0);
    target.relations.0.insert(user.id, status.1);

    let mut tx = pool().begin().await?;

    user.update_tx(&mut tx).await?;
    target.update_tx(&mut tx).await?;

    tx.commit().await?;

    user.relationship = status.1.into();
    target.relationship = status.0.into();

    Payload::UserUpdate(target.clone()).to(user.id).await;
    Payload::UserUpdate(user).to(target.id).await;

    Ok(())
}
