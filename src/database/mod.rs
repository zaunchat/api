use rbatis::rbatis::Rbatis;
use std::env;

lazy_static! {
    pub static ref DB: Rbatis = Rbatis::new();
}

pub async fn connect() {
    let uri = env::var("DATABASE_URI").expect("DATABASE_URI is required");
    DB.link(uri.as_str()).await.unwrap();
}
