use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(Extension(mut user): Extension<User>, Path(id): Path<i64>) -> Result<()> {
    match user.relations.0.get(&id) {
        Some(&status) => {
            let mut target = id.user().await?;
            let target_status = target.relations.0.get(&user.id).unwrap();

            if status != RelationshipStatus::BlockedByOther {
                if target_status == &RelationshipStatus::Blocked {
                    user.relations
                        .0
                        .insert(target.id, RelationshipStatus::BlockedByOther);
                } else {
                    target.relations.0.remove(&user.id);
                    user.relations.0.remove(&target.id);
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
            }

            Ok(())
        }
        _ => Err(Error::UnknownUser),
    }
}
