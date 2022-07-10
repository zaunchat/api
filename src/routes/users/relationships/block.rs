use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn block(Extension(mut user): Extension<User>, Path(id): Path<i64>) -> Result<()> {
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

    let mut user = user
        .update_partial()
        .relations(user.relations.clone())
        .update(&mut tx)
        .await?;

    let mut target = target
        .update_partial()
        .relations(target.relations.clone())
        .update(&mut tx)
        .await?;

    tx.commit().await?;

    user.relationship = status.1.into();
    target.relationship = status.0.into();

    publish(user.id, Payload::UserUpdate(target.clone())).await;
    publish(target.id, Payload::UserUpdate(user)).await;

    Ok(())
}
