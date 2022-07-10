use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn add(Extension(mut user): Extension<User>, Path(id): Path<i64>) -> Result<()> {
    if let Some(&status) = user.relations.0.get(&id) {
        match status {
            RelationshipStatus::Friend => return Err(Error::AlreadyFriends),
            RelationshipStatus::Blocked => return Err(Error::Blocked),
            RelationshipStatus::BlockedByOther => return Err(Error::BlockedByOther),
            RelationshipStatus::Outgoing => return Err(Error::AlreadySendRequest),
            _ => {}
        };
    }

    let mut target = id.user().await?;

    // (user_status, target_status)
    let status = if Some(&RelationshipStatus::Outgoing) == target.relations.0.get(&user.id) {
        // Accept friend request
        (RelationshipStatus::Friend, RelationshipStatus::Friend)
    } else {
        // Send friend request
        (RelationshipStatus::Outgoing, RelationshipStatus::Incoming)
    };

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
