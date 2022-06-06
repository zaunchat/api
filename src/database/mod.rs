use crate::config::DATABASE_URI;
use rbatis::rbatis::Rbatis;

lazy_static! {
    pub static ref DB: Rbatis = Rbatis::new();
}

pub async fn connect() {
    DB.link((*DATABASE_URI).as_str())
        .await
        .expect("Couldn't connect to database");
}
