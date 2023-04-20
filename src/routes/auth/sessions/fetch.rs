use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_one(
    Extension(user): Extension<User>,
    Path(id): Path<Snowflake>,
) -> Result<Json<Session>> {
    Ok(id.session(user.id).await?.into())
}

pub async fn fetch_many(Extension(user): Extension<User>) -> Result<Json<Vec<Session>>> {
    Ok(Session::find("user_id = $1", vec![user.id]).await?.into())
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
            let user = session.user_id.unwrap().user().await?;

            let results = fetch_many(Extension(user.clone())).await?;

            assert_eq!(results.0.len(), 1);

            let _ = fetch_one(Extension(user), Path(session.id)).await?;

            Ok(())
        })
    }
}
