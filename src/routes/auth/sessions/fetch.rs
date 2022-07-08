use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_one(
    Extension(user): Extension<User>,
    Path(id): Path<i64>,
) -> Result<Json<Session>> {
    Ok(Json(id.session(user.id).await?))
}

pub async fn fetch_many(Extension(user): Extension<User>) -> Result<Json<Vec<Session>>> {
    Ok(Json(
        Session::select()
            .filter("user_id = $1")
            .bind(user.id)
            .fetch_all(pool())
            .await?,
    ))
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
            let user = session.user_id.user().await.unwrap();

            let results = fetch_many(Extension(user.clone())).await.unwrap();

            assert_eq!(results.0.len(), 1);

            fetch_one(Extension(user), Path(session.id)).await.unwrap();

            session.cleanup().await;
        })
    }
}
