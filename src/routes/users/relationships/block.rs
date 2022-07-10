use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn block(Extension(mut user): Extension<User>, Path(id): Path<i64>) -> Result<()> {
    let mut target = id.user().await?;

    if let Some(&status) = user.relations.0.get(&id) {
        if status == RelationshipStatus::Blocked {
            return Ok(());
        }

        if status == RelationshipStatus::BlockedByOther {
            user.relations.0.insert(id, RelationshipStatus::Blocked);
        } else {
            user.relations.0.insert(id, RelationshipStatus::Blocked);
            target
                .relations
                .0
                .insert(user.id, RelationshipStatus::BlockedByOther);
        }
    } else {
        user.relations.0.insert(id, RelationshipStatus::Blocked);
        target
            .relations
            .0
            .insert(user.id, RelationshipStatus::BlockedByOther);
    }

    let mut tx = pool().begin().await?;

    user.update_partial()
        .relations(user.relations.clone())
        .update(&mut tx)
        .await?;
    target
        .update_partial()
        .relations(target.relations.clone())
        .update(&mut tx)
        .await?;

    tx.commit().await?;

    Ok(())
}
