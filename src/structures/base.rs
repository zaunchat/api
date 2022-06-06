use crate::database::DB as db;
use async_trait::async_trait;
use rbatis::crud::{CRUDTable, CRUD};
use rbatis::wrapper::Wrapper;
use rocket::serde::DeserializeOwned;

#[async_trait]
pub trait Base: CRUDTable + DeserializeOwned {
    async fn find<F>(query: F) -> Vec<Self>
    where
        F: Send + Fn(Wrapper) -> Wrapper,
    {
        let query = query(db.new_wrapper());
        db.fetch_list_by_wrapper(query).await.unwrap()
    }

    async fn find_one<F>(query: F) -> Option<Self>
    where
        F: Send + Fn(Wrapper) -> Wrapper,
    {
        let query = query(db.new_wrapper());
        db.fetch_by_wrapper(query).await.ok()
    }

    async fn save(&self) {
        db.save(&self, &[])
            .await
            .expect("Couldn't save this target");
    }

    async fn delete(&self, id: i64) -> bool {
        db.remove_by_column::<Self, &i64>("id", &id).await.is_ok()
    }
}
