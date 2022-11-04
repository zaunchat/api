use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(Extension(mut user): Extension<User>, Path(id): Path<Snowflake>) -> Result<()> {
    let status = match user.relations.0.get(&id) {
        Some(s) => s,
        None => return Err(Error::UnknownUser),
    };

    // He blocked you. you can't remove it by yourself
    if status != &RelationshipStatus::BlockedByOther {
        let mut target = id.user().await?;

        if target.relations.0.get(&user.id).unwrap() == &RelationshipStatus::Blocked {
            // If you trying to unblock him but he also blocked you thats will happen
            user.relations
                .0
                .insert(target.id, RelationshipStatus::BlockedByOther);
        } else {
            target.relations.0.remove(&user.id);
            user.relations.0.remove(&target.id);
        }

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

        user.relationship = target.relations.0.get(&user.id).copied();
        target.relationship = user.relations.0.get(&target.id).copied();

        Payload::UserUpdate(target.clone()).to(user.id).await;
        Payload::UserUpdate(user).to(target.id).await;
    }

    Ok(())
}
