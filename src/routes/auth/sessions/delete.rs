use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct DeleteSessionOptions {
    token: String,
}

pub async fn delete(
    Extension(user): Extension<User>,
    Path(id): Path<Snowflake>,
    ValidatedJson(data): ValidatedJson<DeleteSessionOptions>,
) -> Result<()> {
    let session = id.session(user.id).await?;

    if session.token != data.token {
        return Err(Error::InvalidToken);
    }

    session.delete().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;

    #[test]
    fn execute() -> Result<(), Error> {
        run(async {
            let session = Session::faker().await?;
            session.insert().await?;
            let payload = DeleteSessionOptions {
                token: session.token.clone(),
            };

            delete(
                Extension(session.user_id.unwrap().user().await?),
                Path(session.id),
                ValidatedJson(payload),
            )
            .await?;

            Ok(())
        })
    }
}
