use crate::database::DB as db;
use async_trait::async_trait;
use rbatis::crud::{CRUDTable, CRUD};
use rbatis::wrapper::Wrapper;
use rocket::serde::DeserializeOwned;

#[async_trait]
pub trait Base {
    async fn find<F>(query: F) -> Vec<Self>
    where
        Self: CRUDTable + DeserializeOwned,
        F: Send + Fn(Wrapper) -> Wrapper,
    {
        let query = query(db.new_wrapper());
        db.fetch_list_by_wrapper(query).await.unwrap()
    }

    async fn find_one<F>(query: F) -> Self
    where
        Self: CRUDTable + DeserializeOwned,
        F: Send + Fn(Wrapper) -> Wrapper,
    {
        let query = query(db.new_wrapper());
        db.fetch_by_wrapper(query).await.unwrap()
    }

    async fn save(&self)
    where
        Self: CRUDTable + DeserializeOwned,
    {
        db.save(&self, &[]).await.unwrap();
    }

    // TODO: Implement that in here.
    async fn delete(&self) -> bool {
        false
    }

    // async fn delete(&self) -> bool
    // {
    //     db.remove_by_column("id", &self.id).await.is_ok()
    // }
}
