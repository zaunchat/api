use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use crypto::util::fixed_time_eq;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct DeleteSessionOptions {
    token: String,
}

pub async fn delete(
    Extension(user): Extension<User>,
    Path(id): Path<i64>,
    ValidatedJson(data): ValidatedJson<DeleteSessionOptions>,
) -> Result<()> {
    let session = id.session(user.id).await?;

    if !fixed_time_eq(session.token.as_bytes(), data.token.as_bytes()) {
        return Err(Error::InvalidToken);
    }

    session.remove().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;

    #[test]
    fn execute() {
        run(async {
            let session = Session::faker().await;
            let session = session.save().await.unwrap();
            let payload = DeleteSessionOptions {
                token: session.token.clone(),
            };

            delete(
                Extension(session.user_id.user().await.unwrap()),
                Path(session.id),
                ValidatedJson(payload),
            )
            .await
            .unwrap();

            session.cleanup().await.unwrap();
        })
    }
}
