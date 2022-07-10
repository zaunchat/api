use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn add(Extension(mut user): Extension<User>, Path(id): Path<i64>) -> Result<()> {
    if let Some(&status) = user.relations.0.get(&id) {
        if status == RelationshipStatus::Friend {
            return Err(Error::AlreadyFriends);
        }

        if status == RelationshipStatus::Blocked {
            return Err(Error::Blocked);
        }

        if status == RelationshipStatus::BlockedByOther {
            return Err(Error::BlockedByOther);
        }

        if status == RelationshipStatus::Outgoing {
            return Err(Error::AlreadySendRequest);
        }
    }

    let mut target = id.user().await?;

    if let Some(&target_status) = target.relations.0.get(&user.id) {
        if target_status == RelationshipStatus::Outgoing {
            // Accept friend request
            target
                .relations
                .0
                .insert(user.id, RelationshipStatus::Friend);
            user.relations
                .0
                .insert(target.id, RelationshipStatus::Friend);
        }
    } else {
        // Send friend request
        target
            .relations
            .0
            .insert(user.id, RelationshipStatus::Incoming);
        user.relations
            .0
            .insert(target.id, RelationshipStatus::Outgoing);
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
