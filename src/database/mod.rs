use crate::config::DATABASE_URI;
use once_cell::sync::Lazy;
use rbatis::rbatis::Rbatis;

pub static DB: Lazy<Rbatis> = Lazy::new(|| Rbatis::new());

pub async fn connect() {
    DB.link((*DATABASE_URI).as_str())
        .await
        .expect("Couldn't connect to database");
}
